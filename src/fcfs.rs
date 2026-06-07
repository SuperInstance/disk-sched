//! First Come, First Served disk scheduling.

use crate::metrics::SchedMetrics;
use crate::DiskScheduler;

/// FCFS scheduler — services requests in arrival order.
#[derive(Debug, Default)]
pub struct FcfsScheduler;

impl DiskScheduler for FcfsScheduler {
    fn schedule(&self, head: u32, requests: &[u32]) -> SchedMetrics {
        let mut order = vec![head];
        order.extend_from_slice(requests);
        SchedMetrics::from_order(order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_requests() {
        let m = FcfsScheduler.schedule(50, &[]);
        assert_eq!(m.order, vec![50]);
        assert_eq!(m.total_seek, 0);
    }

    #[test]
    fn single_request() {
        let m = FcfsScheduler.schedule(50, &[80]);
        assert_eq!(m.order, vec![50, 80]);
        assert_eq!(m.total_seek, 30);
    }

    #[test]
    fn textbook_example() {
        // Classic textbook: head=53, requests=[98,183,37,122,14,124,65,67]
        let m = FcfsScheduler.schedule(53, &[98, 183, 37, 122, 14, 124, 65, 67]);
        assert_eq!(m.order[0], 53);
        assert_eq!(m.total_seek, 640);
    }

    #[test]
    fn preserves_order() {
        let m = FcfsScheduler.schedule(0, &[5, 3, 1]);
        assert_eq!(m.order, vec![0, 5, 3, 1]);
    }
}
