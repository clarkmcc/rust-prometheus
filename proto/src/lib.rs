mod proto_model;

pub use crate::proto_model::{
    Counter, Gauge, Histogram, LabelPair, Metric, MetricFamily, MetricType, Quantile, Summary,
    Text, Untyped, Bucket
};
use std::cmp::Ordering;

impl Ord for LabelPair {
    fn cmp(&self, other: &LabelPair) -> Ordering {
        self.name().cmp(other.name())
    }
}

impl Eq for LabelPair {}

impl PartialOrd for LabelPair {
    fn partial_cmp(&self, other: &LabelPair) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
