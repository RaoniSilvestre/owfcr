use std::time::{Duration, Instant};

use tracing::info;

/// Interval that the VNS will log to the stdout the progress of the execution.
const LOG_INTERVAL: Duration = Duration::from_secs(30);

pub fn log_progress(
    iters: usize,
    max_iters: usize,
    last_log: &mut Instant,
    current: f64,
    initial: f64,
) {
    if last_log.elapsed() >= LOG_INTERVAL {
        let improvement_pct = ((initial - current) / initial) * 100.0;

        info!(
            "Progress [{} / {} | Best cost: {:.2} | Improvement: {:.2}%",
            iters, max_iters, current, improvement_pct
        );
        *last_log = Instant::now();
    }
}

pub fn log_result(iters: usize, max_iters: usize, current: f64, initial: f64) {
    let improvement_pct = ((initial - current) / initial) * 100.0;

    info!(
        "Progress [{} / {} | Best cost: {:.2} | Improvement: {:.2}%",
        iters, max_iters, current, improvement_pct
    );
    info!("Execution finalized.");
}
