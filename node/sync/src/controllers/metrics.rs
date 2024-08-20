use std::sync::Arc;

use metrics::{register_timer, Counter, CounterUsize, Histogram, Sample, Timer};

lazy_static::lazy_static! {
    pub static ref SERIAL_SYNC_FILE_COMPLETED: Arc<dyn Timer> = register_timer("sync_controllers_serial_sync_file_completed");
    pub static ref SERIAL_SYNC_CHUNKS_COMPLETED: Arc<dyn Timer> = register_timer("sync_controllers_serial_sync_chunks_completed");
    pub static ref SERIAL_SYNC_SEGMENT_LATENCY: Arc<dyn Histogram> = Sample::ExpDecay(0.015).register("sync_controllers_serial_sync_segment_latency", 1024);
    pub static ref SERIAL_SYNC_UNEXPECTED_ERRORS: Arc<dyn Counter<usize>> = CounterUsize::register("sync_controllers_serial_sync_unexpected_errors");
}
