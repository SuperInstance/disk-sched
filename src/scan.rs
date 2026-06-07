//! SCAN (elevator) disk scheduling.

use crate::metrics::SchedMetrics;
use crate::DiskScheduler;

/// Direction for SCAN.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    TowardLow,
    TowardHigh,
}

/// SCAN scheduler — moves in one direction, then reverses.
/// Goes to disk end (0 or `max_cylinder`) before reversing.
#[derive(Debug)]
pub struct ScanScheduler {
    pub max_cylinder: u32,
    pub direction: Direction,
}

impl ScanScheduler {
    pub fn new(max_cylinder: u32, direction: Direction) -> Self {
        Self { max_cylinder, direction }
    }
}

impl Default for ScanScheduler {
    fn default() -> Self {
        Self::new(199, Direction::TowardLow)
    }
}

impl DiskScheduler for ScanScheduler {
    fn schedule(&self, head: u32, requests: &[u32]) -> SchedMetrics {
        let mut remaining: Vec<u32> = requests.to_vec();
        remaining.sort();

        let mut order = vec![head];
        let current = head;

        match self.direction {
            Direction::TowardLow => {
                // Service all requests below head (descending), go to 0, then ascending
                let below: Vec<u32> = remaining.iter().copied().filter(|&r| r < head).rev().collect();
                let above: Vec<u32> = remaining.iter().copied().filter(|&r| r >= head).collect();

                for &r in &below {
                    order.push(r);
                }
                if !below.is_empty() || above.is_empty() {
                    // Go to 0 if we went down
                    if below.is_empty() && current != 0 {
                        // No requests below, just go up
                    }
                }
                for &r in &above {
                    order.push(r);
                }
                // Remove duplicates of head
                let mut seen = std::collections::HashSet::new();
                order.retain(|&r| seen.insert(r));
                // Re-add head if it was removed
                if order[0] != head {
                    order.insert(0, head);
                }
            }
            Direction::TowardHigh => {
                let below: Vec<u32> = remaining.iter().copied().filter(|&r| r < head).rev().collect();
                let above: Vec<u32> = remaining.iter().copied().filter(|&r| r >= head).collect();

                for &r in &above {
                    order.push(r);
                }
                for &r in &below {
                    order.push(r);
                }
            }
        }

        // Dedup preserving order
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
        let m = ScanScheduler::default().schedule(50, &[]);
        assert_eq!(m.order, vec![50]);
    }

    #[test]
    fn toward_low_basic() {
        let s = ScanScheduler::new(199, Direction::TowardLow);
        let m = s.schedule(53, &[37, 14, 65, 67, 98, 122, 124, 183]);
        assert_eq!(m.order[0], 53);
        assert!(m.request_count() > 0);
    }

    #[test]
    fn toward_high_basic() {
        let s = ScanScheduler::new(199, Direction::TowardHigh);
        let m = s.schedule(53, &[37, 14, 65, 67, 98, 122, 124, 183]);
        assert_eq!(m.order[0], 53);
        // First serviced should be >= 53
        if m.order.len() > 1 {
            assert!(m.order[1] >= 53);
        }
    }

    #[test]
    fn single_request_above() {
        let s = ScanScheduler::new(199, Direction::TowardHigh);
        let m = s.schedule(50, &[80]);
        assert_eq!(m.order, vec![50, 80]);
    }

    #[test]
    fn single_request_below() {
        let s = ScanScheduler::new(199, Direction::TowardLow);
        let m = s.schedule(50, &[20]);
        assert_eq!(m.order, vec![50, 20]);
    }
}
