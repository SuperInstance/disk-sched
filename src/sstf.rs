//! Shortest Seek Time First disk scheduling.

use crate::metrics::SchedMetrics;
use crate::DiskScheduler;

/// SSTF scheduler — always services the closest request next.
#[derive(Debug, Default)]
pub struct SstfScheduler;

impl DiskScheduler for SstfScheduler {
    fn schedule(&self, head: u32, requests: &[u32]) -> SchedMetrics {
        let mut remaining: Vec<u32> = requests.to_vec();
        let mut order = vec![head];
        let mut current = head;

        while !remaining.is_empty() {
            let (idx, _) = remaining
                .iter()
                .enumerate()
                .min_by_key(|(_, &r)| current.abs_diff(r))
                .unwrap();
            current = remaining.remove(idx);
            order.push(current);
        }

        SchedMetrics::from_order(order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_requests() {
        let m = SstfScheduler.schedule(50, &[]);
        assert_eq!(m.order, vec![50]);
        assert_eq!(m.total_seek, 0);
    }

    #[test]
    fn single_request() {
        let m = SstfScheduler.schedule(50, &[80]);
        assert_eq!(m.order, vec![50, 80]);
    }

    #[test]
    fn picks_closest() {
        let m = SstfScheduler.schedule(50, &[90, 20, 80]);
        // Closest to 50: 20(dist 30) or 80(dist 30), tie → first in vec is 90, then 20 (dist 30), then 80
        // Actually min_by_key picks first min: 20 (dist 30, comes before 80 in iteration)
        assert_eq!(m.order[1], 20);
    }

    #[test]
    fn textbook_example() {
        let m = SstfScheduler.schedule(53, &[98, 183, 37, 122, 14, 124, 65, 67]);
        assert!(m.total_seek < 640); // Must be better than FCFS
        assert_eq!(m.request_count(), 8);
    }

    #[test]
    fn all_equal_distance() {
        let m = SstfScheduler.schedule(50, &[40, 60]);
        // Both distance 10, picks first found
        assert_eq!(m.order.len(), 3);
    }
}
