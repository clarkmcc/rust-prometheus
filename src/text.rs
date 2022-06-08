use std::sync::Arc;
use crate::core::{Collector, Desc, Metric, MetricVec, MetricVecBuilder};
use crate::{Opts};
use crate::value::{StringValue, ValueType};
use crate::errors::Result;

/// Represents a [`MetricVec`] that can hold string values with associated labels
/// just like a counter or gauge vector metric.
pub type TextVec = MetricVec<TextVecBuilder>;

impl TextVec {
    /// Create a new [`TextVec`] based on the provided
    /// [`Opts`] and partitioned by the given label names. At least one label name must
    /// be provided.
    pub fn new(opts: Opts, label_names: &[&str]) -> Result<Self> {
        let variable_names = label_names.iter().map(|s| (*s).to_owned()).collect();
        let opts = opts.variable_labels(variable_names);
        let metric_vec = MetricVec::create(proto::MetricType::TEXT, TextVecBuilder::new(), opts)?;
        Ok(metric_vec as Self)
    }
}

#[derive(Debug)]
pub struct TextVecBuilder {}

impl TextVecBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Clone for TextVecBuilder {
    fn clone(&self) -> Self {
        Self::new()
    }
}

impl MetricVecBuilder for TextVecBuilder {
    type M = TextValue;
    type P = Opts;

    fn build(&self, opts: &Opts, vals: &[&str]) -> Result<Self::M> {
        Self::M::with_opts_and_label_values(opts, vals)
    }
}


/// The underlying implementation for [`Gauge`] and [`IntGauge`].
#[derive(Debug)]
pub struct TextValue {
    v: Arc<StringValue>,
}

impl Clone for TextValue {
    fn clone(&self) -> Self {
        Self {
            v: Arc::clone(&self.v),
        }
    }
}

impl TextValue {
    /// Create a [`GenericCounter`] with the `name` and `help` arguments.
    pub fn new<S1: Into<String>, S2: Into<String>>(name: S1, help: S2) -> Result<Self> {
        let opts = Opts::new(name, help);
        Self::with_opts(opts)
    }

    /// Create a [`GenericCounter`] with the `opts` options.
    pub fn with_opts(opts: Opts) -> Result<Self> {
        Self::with_opts_and_label_values(&opts, &[])
    }

    fn with_opts_and_label_values(opts: &Opts, label_values: &[&str]) -> Result<Self> {
        let v = StringValue::new(opts, ValueType::Text, String::from(""), label_values)?;
        Ok(Self { v: Arc::new(v) })
    }

    /// Return the counter value.
    #[inline]
    pub fn get(&self) -> String {
        self.v.get()
    }

    /// Set the string to an arbitrary value.
    #[inline]
    pub fn set<S>(&self, v: S) where S: Into<String> {
        self.v.set(v.into());
    }

    /// Restart the counter, resetting its value back to 0.
    #[inline]
    pub fn reset(&self) {
        self.v.set(String::from(""))
    }

    /// Return a [`GenericLocalCounter`] for single thread usage.
    pub fn local(&self) -> TextValue {
        self.clone()
    }
}

impl Collector for TextValue {
    fn desc(&self) -> Vec<&Desc> {
        vec![&self.v.desc]
    }

    fn collect(&self) -> Vec<proto::MetricFamily> {
        vec![self.v.collect()]
    }
}

impl Metric for TextValue {
    fn metric(&self) -> proto::Metric {
        self.v.metric()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::metrics::{Collector, Opts};

    #[test]
    fn test_text_value() {
        let opts = Opts::new("test", "test help")
            .const_label("a", "1")
            .const_label("b", "2");
        let counter = TextValue::with_opts(opts).unwrap();
        counter.set("foo");
        assert_eq!(counter.get(), "foo");
    }

    #[test]
    fn test_text_vec() {
        let opts = Opts::new("test", "help");
        let vec = TextVec::new(opts, &["format"]).unwrap();
        vec.with_label_values(&["engine"]).set("foo");
        vec.with_label_values(&["life"]).set("bar");

        let mf = vec.collect();
        assert_eq!(mf[0].metric.len(), 2)
    }
}