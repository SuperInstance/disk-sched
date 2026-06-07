//! Disk scheduling algorithms.
//!
//! Provides FCFS, SSTF, SCAN, C-SCAN, and LOOK scheduling strategies
//! with seek-time metrics.

pub mod fcfs;
pub mod sstf;
pub mod scan;
pub mod look;
pub mod metrics;

pub use metrics::SchedMetrics;

/// Trait for disk schedulers.
pub trait DiskScheduler {
    /// Schedule requests starting from `head` position.
    /// Returns the service order and metrics.
    fn schedule(&self, head: u32, requests: &[u32]) -> SchedMetrics;
}
