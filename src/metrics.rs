//! Metrics for disk scheduling results.

/// Result metrics from a disk scheduling algorithm.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SchedMetrics {
    /// Service order (including initial head position as first element).
    pub order: Vec<u32>,
    /// Total seek distance.
    pub total_seek: u32,
}

impl SchedMetrics {
    /// Build metrics from a service order (first element = starting head).
    pub fn from_order(order: Vec<u32>) -> Self {
        let total_seek = order.windows(2).map(|w| w[0].abs_diff(w[1])).sum();
        Self { order, total_seek }
    }

    /// Average seek distance.
    pub fn average_seek(&self) -> f64 {
        if self.order.len() <= 1 {
            0.0
        } else {
            self.total_seek as f64 / (self.order.len() - 1) as f64
        }
    }

    /// Maximum single seek.
    pub fn max_seek(&self) -> u32 {
        self.order
            .windows(2)
            .map(|w| w[0].abs_diff(w[1]))
            .max()
            .unwrap_or(0)
    }

    /// Number of requests serviced.
    pub fn request_count(&self) -> usize {
        if self.order.is_empty() { 0 } else { self.order.len() - 1 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let m = SchedMetrics::from_order(vec![]);
        assert_eq!(m.total_seek, 0);
        assert_eq!(m.average_seek(), 0.0);
    }

    #[test]
    fn single_position() {
        let m = SchedMetrics::from_order(vec![50]);
        assert_eq!(m.total_seek, 0);
        assert_eq!(m.request_count(), 0);
    }

    #[test]
    fn linear_seeks() {
        let m = SchedMetrics::from_order(vec![50, 60, 70, 80]);
        assert_eq!(m.total_seek, 30);
        assert!((m.average_seek() - 10.0).abs() < f64::EPSILON);
        assert_eq!(m.max_seek(), 10);
    }

    #[test]
    fn non_monotonic() {
        let m = SchedMetrics::from_order(vec![50, 10, 90]);
        assert_eq!(m.total_seek, 120); // 40 + 80
        assert_eq!(m.max_seek(), 80);
    }

    #[test]
    fn request_count() {
        let m = SchedMetrics::from_order(vec![50, 60, 70]);
        assert_eq!(m.request_count(), 2);
    }
}
