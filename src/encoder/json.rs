use std::io;
use std::io::{Write};
use protobuf::text_format::print_to_string;
use proto::MetricFamily;
use crate::Encoder;
use crate::errors::Result;

#[derive(Default)]
pub struct JSONEncoder;

pub const JSON_FORMAT: &str = "application/json; version=0.0.1";

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
        self.encode(metric_families, writer)
    }
}

impl Encoder for JSONEncoder {
    fn encode<W: Write>(&self, mfs: &[MetricFamily], writer: &mut W) -> crate::Result<()> {
        let string = print_to_string(mfs).unwrap();
        // let str = serde_json::to_string(mfs).map_err(||Err(()))?;
        writer.write(string).unwrap();
        Ok(())
    }

    fn format_type(&self) -> &str {
        JSON_FORMAT
    }
}

trait WriteUtf8 {
    fn write_all(&mut self, text: &str) -> io::Result<()>;
}

impl<W: Write> WriteUtf8 for W {
    fn write_all(&mut self, text: &str) -> io::Result<()> {
        Write::write_all(self, text.as_bytes())
    }
}

/// Coherence forbids to impl `WriteUtf8` directly on `String`, need this
/// wrapper as a work-around.
struct StringBuf<'a>(&'a mut String);

impl WriteUtf8 for StringBuf<'_> {
    fn write_all(&mut self, text: &str) -> io::Result<()> {
        self.0.push_str(text);
        Ok(())
    }
}

mod tests {
    use crate::encoder::json::JSONEncoder;
    use crate::{Counter, CounterVec, Encoder, Opts, Registry};

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

        let counter_ans = r##"# HELP test_counter test help
# TYPE test_counter counter
test_counter{a="1",b="2"} 1
"##;
        assert_eq!(counter_ans, txt.as_str());
    }
}