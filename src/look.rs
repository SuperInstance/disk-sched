//! LOOK disk scheduling (SCAN variant that doesn't go to disk end).

use crate::metrics::SchedMetrics;
use crate::DiskScheduler;

/// Direction for LOOK.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LookDirection {
    TowardLow,
    TowardHigh,
}

/// LOOK scheduler — like SCAN but reverses at last request instead of disk end.
#[derive(Debug)]
pub struct LookScheduler {
    pub direction: LookDirection,
}

impl LookScheduler {
    pub fn new(direction: LookDirection) -> Self {
        Self { direction }
    }
}

impl Default for LookScheduler {
    fn default() -> Self {
        Self::new(LookDirection::TowardHigh)
    }
}

impl DiskScheduler for LookScheduler {
    fn schedule(&self, head: u32, requests: &[u32]) -> SchedMetrics {
        let mut sorted: Vec<u32> = requests.to_vec();
        sorted.sort();

        let below: Vec<u32> = sorted.iter().copied().filter(|&r| r < head).collect();
        let above: Vec<u32> = sorted.iter().copied().filter(|&r| r >= head).collect();

        let mut order = vec![head];

        match self.direction {
            LookDirection::TowardHigh => {
                for &r in &above {
                    order.push(r);
                }
                for &r in below.iter().rev() {
                    order.push(r);
                }
            }
            LookDirection::TowardLow => {
                for &r in below.iter().rev() {
                    order.push(r);
                }
                for &r in &above {
                    order.push(r);
                }
            }
        }

        // Dedup
        let mut unique = Vec::new();
        let mut seen = std::collections::HashSet::new();
        for &r in &order {
            if seen.insert(r) {
                unique.push(r);
            }
        }

        SchedMetrics::from_order(unique)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_requests() {
        let m = LookScheduler::default().schedule(50, &[]);
        assert_eq!(m.order, vec![50]);
    }

    #[test]
    fn toward_high_ascending() {
        let m = LookScheduler::new(LookDirection::TowardHigh).schedule(50, &[60, 70, 40, 30]);
        // Above first: 60, 70. Then below reversed: 40, 30
        assert_eq!(m.order, vec![50, 60, 70, 40, 30]);
    }

    #[test]
    fn toward_low_descending() {
        let m = LookScheduler::new(LookDirection::TowardLow).schedule(50, &[60, 70, 40, 30]);
        // Below first (reversed): 40, 30. Then above: 60, 70
        assert_eq!(m.order, vec![50, 40, 30, 60, 70]);
    }

    #[test]
    fn look_vs_scan_no_end_travel() {
        let look = LookScheduler::new(LookDirection::TowardHigh);
        let m = look.schedule(50, &[55]);
        // LOOK should just go 50->55, not to disk end
        assert_eq!(m.order, vec![50, 55]);
        assert_eq!(m.total_seek, 5);
    }

    #[test]
    fn all_above() {
        let m = LookScheduler::new(LookDirection::TowardHigh).schedule(0, &[10, 20, 30]);
        assert_eq!(m.order, vec![0, 10, 20, 30]);
    }

    #[test]
    fn all_below() {
        let m = LookScheduler::new(LookDirection::TowardHigh).schedule(100, &[10, 20, 30]);
        // Above (none), then below reversed: 30, 20, 10
        assert_eq!(m.order, vec![100, 30, 20, 10]);
    }
}
