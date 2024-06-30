use prometheus::HistogramOpts;
use prometheus::Opts;
use std::time::Duration;

pub use prometheus::{
    exponential_buckets, linear_buckets,
    proto::{Metric, MetricFamily, MetricType},
    Encoder, Gauge, GaugeVec, Histogram, HistogramTimer, HistogramVec, IntCounter, IntCounterVec,
    IntGauge, IntGaugeVec, Result, TextEncoder, DEFAULT_BUCKETS,
};

pub fn gather() -> Vec<prometheus::proto::MetricFamily> {
    prometheus::gather()
}

pub fn try_create_int_counter(name: &str, help: &str) -> Result<IntCounter> {
    let opts = Opts::new(name, help);
    let counter = IntCounter::with_opts(opts)?;
    prometheus::register(Box::new(counter.clone()))?;
    Ok(counter)
}

pub fn try_create_int_gauge(name: &str, help: &str) -> Result<IntGauge> {
    let opts = Opts::new(name, help);
    let gauge = IntGauge::with_opts(opts)?;
    prometheus::register(Box::new(gauge.clone()))?;
    Ok(gauge)
}

pub fn try_create_histogram(name: &str, help: &str) -> Result<Histogram> {
    let opts = HistogramOpts::new(name, help);
    let histogram = Histogram::with_opts(opts)?;
    prometheus::register(Box::new(histogram.clone()))?;
    Ok(histogram)
}

pub fn try_create_int_gauge_vec(
    name: &str,
    help: &str,
    label_names: &[&str],
) -> Result<IntGaugeVec> {
    let opts = Opts::new(name, help);
    let gauge_vec = IntGaugeVec::new(opts, label_names)?;
    prometheus::register(Box::new(gauge_vec.clone()))?;
    Ok(gauge_vec)
}

pub fn try_create_histogram_vec(
    name: &str,
    help: &str,
    label_names: &[&str],
) -> Result<HistogramVec> {
    let opts = HistogramOpts::new(name, help);
    let histogram_vec = HistogramVec::new(opts, label_names)?;
    prometheus::register(Box::new(histogram_vec.clone()))?;
    Ok(histogram_vec)
}

pub fn inc_counter(counter: &Result<IntCounter>) {
    if let Ok(counter) = counter {
        counter.inc();
    }
}

pub fn set_gauge(gauge: &Result<IntGauge>, value: i64) {
    if let Ok(gauge) = gauge {
        gauge.set(value);
    }
}

pub fn observe(histogram: &Result<Histogram>, value: f64) {
    if let Ok(histogram) = histogram {
        histogram.observe(value);
    }
}

pub fn observe_duration(histogram: &Result<Histogram>, duration: Duration) {
    if let Ok(histogram) = histogram {
        histogram.observe(duration_to_f64(duration))
    }
}

fn duration_to_f64(duration: Duration) -> f64 {
    duration.as_secs() as f64 + f64::from(duration.subsec_nanos()) / 1e9
}
