// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

//! Non-generated version of required structures provided by the protobuf.
//! This version is used when the `protobuf` feature is turned off.

#![allow(missing_docs)]

use std::borrow::Borrow;
use std::cmp::Ordering;
use serde::{Serialize, Deserialize};

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct LabelPair {
    name: String,
    value: String,
}

impl LabelPair {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> LabelPair {
        Default::default()
    }

    #[deprecated(
        note = "This method is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn set_name(&mut self, v: String) {
        self.name = v;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_value(&mut self, v: String) {
        self.value = v;
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

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

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct Gauge {
    value: f64,
}

impl Gauge {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Gauge {
        Default::default()
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct Counter {
    value: f64,
}

impl Counter {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Counter {
        Default::default()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct Quantile {
    quantile: f64,
    value: f64,
}

impl Quantile {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Quantile {
        Default::default()
    }

    pub fn set_quantile(&mut self, v: f64) {
        self.quantile = v;
    }

    pub fn quantile(&self) -> f64 {
        self.quantile
    }

    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct Summary {
    sample_count: u64,
    sample_sum: f64,
    quantile: Vec<Quantile>,
}

impl Summary {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Summary {
        Default::default()
    }

    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = v;
    }

    pub fn sample_count(&self) -> u64 {
        self.sample_count
    }

    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = v;
    }

    pub fn sample_sum(&self) -> f64 {
        self.sample_sum
    }

    pub fn set_quantile(&mut self, v: Vec<Quantile>) {
        self.quantile = v;
    }

    pub fn quantile(&self) -> &[Quantile] {
        &self.quantile
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct Untyped {
    value: f64,
}

impl Untyped {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Untyped {
        Default::default()
    }

    #[deprecated(
        note = "Untyped struct is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn set_value(&mut self, v: f64) {
        self.value = v;
    }

    #[deprecated(
        note = "Untyped struct is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn value(&self) -> f64 {
        self.value
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct Histogram {
    sample_count: u64,
    sample_sum: f64,
    bucket: Vec<Bucket>,
}

impl Histogram {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Histogram {
        Default::default()
    }

    pub fn set_sample_count(&mut self, v: u64) {
        self.sample_count = v;
    }

    pub fn sample_count(&self) -> u64 {
        self.sample_count
    }

    pub fn set_sample_sum(&mut self, v: f64) {
        self.sample_sum = v;
    }

    pub fn sample_sum(&self) -> f64 {
        self.sample_sum
    }

    pub fn set_bucket(&mut self, v: Vec<Bucket>) {
        self.bucket = v;
    }

    pub fn bucket(&self) -> &[Bucket] {
        &self.bucket
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct Text {
    value: String,
}

impl Text {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Text {
        Default::default()
    }

    pub fn set_value(&mut self, v: String) {
        self.value = v;
    }

    pub fn value(&self) -> String {
        self.value.clone()
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct Bucket {
    cumulative_count: u64,
    upper_bound: f64,
}

impl Bucket {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Bucket {
        Default::default()
    }

    pub fn set_cumulative_count(&mut self, v: u64) {
        self.cumulative_count = v;
    }

    pub fn cumulative_count(&self) -> u64 {
        self.cumulative_count
    }

    pub fn set_upper_bound(&mut self, v: f64) {
        self.upper_bound = v;
    }

    pub fn upper_bound(&self) -> f64 {
        self.upper_bound
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct Metric {
    // message fields
    label: Vec<LabelPair>,
    gauge: Option<Gauge>,
    counter: Option<Counter>,
    summary: Option<Summary>,
    untyped: Option<Untyped>,
    histogram: Option<Histogram>,
    text: Option<Text>,
    timestamp_ms: i64,
}

impl Metric {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> Metric {
        Default::default()
    }

    pub fn set_label(&mut self, v: Vec<LabelPair>) {
        self.label = v;
    }

    pub fn mut_label(&mut self) -> &mut [LabelPair] {
        &mut self.label
    }

    pub fn take_label(&mut self) -> Vec<LabelPair> {
        ::std::mem::replace(&mut self.label, Vec::new())
    }

    pub fn label(&self) -> &[LabelPair] {
        &self.label
    }

    pub fn set_gauge(&mut self, v: Gauge) {
        self.gauge = Some(v);
    }

    pub fn gauge(&self) -> &Gauge {
        self.gauge.as_ref().unwrap()
    }

    pub fn set_counter(&mut self, v: Counter) {
        self.counter = Some(v);
    }

    pub fn counter(&self) -> &Counter {
        self.counter.as_ref().unwrap()
    }

    pub fn set_summary(&mut self, v: Summary) {
        self.summary = Some(v);
    }

    pub fn summary(&self) -> &Summary {
        self.summary.as_ref().unwrap()
    }

    #[deprecated(
        note = "This method is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn set_untyped(&mut self, v: Untyped) {
        self.untyped = Some(v);
    }

    #[deprecated(
        note = "This method is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn untyped(&self) -> &Untyped {
        self.untyped.as_ref().unwrap()
    }

    pub fn set_histogram(&mut self, v: Histogram) {
        self.histogram = Some(v);
    }

    pub fn histogram(&self) -> &Histogram {
        self.histogram.as_ref().unwrap()
    }

    pub fn set_text(&mut self, v: Text) {
        self.text = Some(v)
    }

    pub fn text(&self) -> &Text {
        self.text.as_ref().unwrap()
    }

    pub fn set_timestamp_ms(&mut self, v: i64) {
        self.timestamp_ms = v;
    }

    pub fn timestamp_ms(&self) -> i64 {
        self.timestamp_ms
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Copy, Serialize)]
pub enum MetricType {
    COUNTER,
    GAUGE,
    SUMMARY,
    UNTYPED,
    HISTOGRAM,
    TEXT,
}

impl Default for MetricType {
    fn default() -> Self {
        MetricType::COUNTER
    }
}

#[derive(PartialEq, Clone, Default, Debug, Serialize)]
pub struct MetricFamily {
    name: String,
    help: String,
    field_type: MetricType,
    metric: Vec<Metric>,
}

impl MetricFamily {
    #[deprecated(note = "Use default()", since = "0.5.1")]
    pub fn new() -> MetricFamily {
        Default::default()
    }

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn set_name(&mut self, v: String) {
        self.name = v;
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_help(&mut self, v: String) {
        self.help = v;
    }

    pub fn help(&self) -> &str {
        &self.help
    }

    pub fn set_type(&mut self, v: MetricType) {
        self.field_type = v;
    }

    pub fn type_(&self) -> MetricType {
        self.field_type
    }

    #[deprecated(
        note = "This method is protobuf specific and will be removed in a future version",
        since = "0.5.1"
    )]
    pub fn clear_metric(&mut self) {
        self.metric.clear();
    }

    pub fn set_metric(&mut self, v: Vec<Metric>) {
        self.metric = v;
    }

    pub fn mut_metric(&mut self) -> &mut Vec<Metric> {
        &mut self.metric
    }

    pub fn take_metric(&mut self) -> Vec<Metric> {
        ::std::mem::replace(&mut self.metric, Vec::new())
    }

    pub fn metric(&self) -> &[Metric] {
        &self.metric
    }
}
