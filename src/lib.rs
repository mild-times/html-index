#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations)]

const DOCTYPE: &str = "<!DOCTYPE html>";
const CHARSET: &str = "<meta charset=\"utf-8\">";
const VIEWPORT: &str = "<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">";
const HTML_OPEN: &str = "<html>";
const HTML_CLOSE: &str = "</html>";
const HEAD_OPEN: &str = "<head>";
const HEAD_CLOSE: &str = "</head>";

#[derive(Debug, Clone)]
pub struct Builder<'b> {
  body: &'b str,
  desc: Option<String>,
}

impl <'b>Builder<'b> {
  /// Create a new instance from an HTML body, including `<body></body>` tags.
  pub fn new(body: &'b str) -> Self {
    Self { body, desc: None }
  }

  /// Add a description.
  pub fn description(&mut self, desc: &str) {
    let val = format!("<meta name=\"description\" content=\"{}\">", desc);
    self.desc = Some(val);
  }

  /// Finalize the builder.
  pub fn build(self) -> String {
    let mut html: String = DOCTYPE.into();
    html.push_str(HTML_OPEN);
    html.push_str(HEAD_OPEN);
    html.push_str(CHARSET);
    html.push_str(VIEWPORT);
    html.push_str(HEAD_CLOSE);
    html.push_str(self.body);
    html.push_str(HTML_CLOSE);
    html
  }
}
