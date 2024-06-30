use lazy_static::lazy_static;
use prometheus::{HistogramTimer, HistogramVec, IntGaugeVec};
pub use tokenscout_metrics::*;

lazy_static! {
    pub static ref ASYNC_TASKS_COUNT: Result<IntGaugeVec> = try_create_int_gauge_vec(
        "async_tasks_count",
        "Total number of async tasks spawned using spawn",
        &["async_task_count"]
    );
    pub static ref BLOCKING_TASKS_COUNT: Result<IntGaugeVec> = try_create_int_gauge_vec(
        "blocking_tasks_count",
        "Total number of async tasks spawned using spawn_blocking",
        &["blocking_task_count"]
    );
    pub static ref BLOCKING_TASKS_HISTOGRAM: Result<HistogramVec> = try_create_histogram_vec(
        "blocking_tasks_histogram",
        "Time taken by blocking tasks",
        &["blocking_task_hist"]
    );
    pub static ref BLOCK_ON_TASKS_COUNT: Result<IntGaugeVec> = try_create_int_gauge_vec(
        "block_on_tasks_count",
        "Total number of block_on_dangerous tasks spawned",
        &["name"]
    );
    pub static ref BLOCK_ON_TASKS_HISTOGRAM: Result<HistogramVec> = try_create_histogram_vec(
        "block_on_tasks_histogram",
        "Time taken by block_on_dangerous tasks",
        &["name"]
    );
    pub static ref TASKS_HISTOGRAM: Result<HistogramVec> = try_create_histogram_vec(
        "async_tasks_time_histogram",
        "Time taken by async tasks",
        &["async_task_hist"]
    );
}

pub fn start_timer_vec(
    histogram_vec: &Result<HistogramVec>,
    labels: &[&str],
) -> Option<HistogramTimer> {
    if let Ok(histogram_vec) = histogram_vec {
        Some(histogram_vec.with_label_values(labels).start_timer())
    } else {
        None
    }
}

pub fn get_int_gauge(int_gauge_vec: &Result<IntGaugeVec>, labels: &[&str]) -> Option<IntGauge> {
    if let Ok(int_gauge_vec) = int_gauge_vec {
        Some(int_gauge_vec.with_label_values(labels))
    } else {
        None
    }
}
