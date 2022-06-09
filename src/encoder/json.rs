use std::io::{self, Write};
use protobuf::text_format::print_to_string;
use crate::proto::{MetricFamily, MetricType};
use crate::Encoder;
use crate::encoder::check_metric_family;
use crate::encoder::text::{StringBuf, WriteUtf8};
use crate::errors::Result;


pub const JSON_FORMAT: &str = "application/json; version=0.0.1";

/// An implementation of an [`Encoder`] that converts a [`MetricFamily`] proto message
/// into text format.
#[derive(Debug, Default)]
pub struct JSONEncoder;

impl JSONEncoder {
    /// Create a new text encoder.
    pub fn new() -> JSONEncoder {
        JSONEncoder
    }
    /// Appends metrics to a given `String` buffer.
    ///
    /// This is a convenience wrapper around `<TextEncoder as Encoder>::encode`.
    pub fn encode_utf8(&self, metric_families: &[MetricFamily], buf: &mut String) -> Result<()> {
        // Note: it's important to *not* re-validate UTF8-validity for the
        // entirety of `buf`. Otherwise, repeatedly appending metrics to the
        // same `buf` will lead to quadratic behavior. That's why we use
        // `WriteUtf8` abstraction to skip the validation.
        self.encode_impl(metric_families, &mut StringBuf(buf))?;
        Ok(())
    }
    /// Converts metrics to `String`.
    ///
    /// This is a convenience wrapper around `<TextEncoder as Encoder>::encode`.
    pub fn encode_to_string(&self, metric_families: &[MetricFamily]) -> Result<String> {
        let mut buf = String::new();
        self.encode_utf8(metric_families, &mut buf)?;
        Ok(buf)
    }

    fn encode_impl(
        &self,
        metric_families: &[MetricFamily],
        writer: &mut dyn WriteUtf8,
    ) -> Result<()> {
        writer.write_all(serde_json::to_string(metric_families).unwrap().as_str())?;
        Ok(())
    }
}

impl Encoder for JSONEncoder {
    fn encode<W: Write>(&self, metric_families: &[MetricFamily], writer: &mut W) -> Result<()> {
        self.encode_impl(metric_families, &mut *writer)
    }

    fn format_type(&self) -> &str {
        JSON_FORMAT
    }
}

mod tests {
    use crate::encoder::json::JSONEncoder;
    use crate::{Counter, CounterVec, Encoder, Opts, Registry};
    use crate::metrics::Collector;

    #[test]
    fn test() {
        let counter_opts = Opts::new("test_counter", "test help")
            .const_label("a", "1")
            .const_label("b", "2");
        let counter = Counter::with_opts(counter_opts).unwrap();
        counter.inc();

        let mf = counter.collect();

        let encoder = JSONEncoder::new();
        let txt = encoder.encode_to_string(&mf);
        let txt = txt.unwrap();
        let counter_ans = r##"[{"name":"test_counter","help":"test help","field_type":"COUNTER","metric":[{"label":[{"name":"a","value":"1"},{"name":"b","value":"2"}],"gauge":null,"counter":{"value":1.0},"summary":null,"untyped":null,"histogram":null,"text":null,"timestamp_ms":0}]}]"##;
        assert_eq!(counter_ans, txt.as_str());
    }
}