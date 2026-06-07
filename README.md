# disk-sched

Disk scheduling algorithm simulator for research and education.

Implements five classic disk I/O scheduling strategies with seek-time metrics:

- **FCFS** — First Come, First Served
- **SSTF** — Shortest Seek Time First
- **SCAN** — Elevator algorithm
- **C-SCAN** — Circular SCAN
- **LOOK** — SCAN variant that reverses at last request

## Usage

```rust
use disk_sched::{fcfs::FcfsScheduler, metrics::SchedMetrics};

let scheduler = FcfsScheduler;
let metrics: SchedMetrics = scheduler.schedule(50, &[98, 183, 37, 122, 14, 124, 65, 67]);
println!("Total seek: {}", metrics.total_seek);
println!("Order: {:?}", metrics.order);
```

## Metrics

The `metrics` module provides `SchedMetrics` with:
- Total seek distance
- Seek order
- Average seek
- Max seek (span)

No external dependencies — pure `std`.

## License

MIT
