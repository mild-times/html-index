#![cfg_attr(feature = "nightly", deny(missing_docs))]
#![cfg_attr(feature = "nightly", feature(external_doc))]
#![cfg_attr(feature = "nightly", doc(include = "../README.md"))]
#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations)]

const DOCTYPE: &str = "<!DOCTYPE html>";
const CHARSET: &str = r#"<meta charset="utf-8">"#;
const VIEWPORT: &str =
  r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#;
const HTML_OPEN: &str = "<html>";
const HTML_CLOSE: &str = "</html>";
const HEAD_OPEN: &str = "<head>";
const HEAD_CLOSE: &str = "</head>";

/// Create a new HTML builder.
#[derive(Debug, Clone)]
pub struct Builder<'b> {
  color: Option<String>,
  desc: Option<String>,
  favicon: Option<String>,
  fonts: Vec<String>,
  manifest: Option<String>,
  scripts: Vec<String>,
  styles: Vec<String>,
  title: Option<String>,
  body: Option<&'b str>,
}

impl<'b> Builder<'b> {
  /// Create a new instance from an HTML body, including `<body></body>` tags.
  pub fn new() -> Self {
    Self {
      body: None,
      color: None,
      desc: None,
      favicon: None,
      fonts: vec![],
      manifest: None,
      scripts: vec![],
      styles: vec![],
      title: None,
    }
  }

  /// Add a body to the document. The body must include `<body></body>` tags.
  pub fn raw_body(mut self, body: &'b str) -> Self {
    self.body = Some(body);
    self
  }

  /// Add a `<meta name="description">` tag.
  pub fn description(mut self, desc: &str) -> Self {
    let val = format!(r#"<meta name="description" content="{}">"#, desc);
    self.desc = Some(val);
    self
  }

  /// Add a `<meta name="theme-color">` tag.
  pub fn theme_color(mut self, color: &str) -> Self {
    let val = format!(r#"<meta name="theme-color" content="{}">"#, color);
    self.color = Some(val);
    self
  }

  /// Add a `<title>` tag.
  pub fn title(mut self, title: &str) -> Self {
    let val = format!(r#"<title>{}</title>"#, title);
    self.title = Some(val);
    self
  }

  /// Add a `<script defer>` tag. This is ideal for loading scripts that are
  /// important for the main application, but shouldn't interfere with the
  /// initial rendering.
  // TODO: also allow passing a sha512
  pub fn script(mut self, src: &str) -> Self {
    let val = format!(r#"<script src="{}" defer></script>"#, src);
    self.scripts.push(val);
    self
  }

  /// Add a `<link rel="prefetch">` tag. This is ideal for loading scripts in
  /// the background after the main application has loaded.
  // TODO: also allow passing a sha512
  pub fn lazy_script(mut self, src: &str) -> Self {
    let val = format!(r#"<link rel="prefetch" href="{}">"#, src);
    self.scripts.push(val);
    self
  }

  /// Add a `<script>` tag. This is ideal for loading scripts that should be
  /// loaded before any rendering can start.
  // TODO: also allow passing a sha512
  pub fn blocking_script(mut self, src: &str) -> Self {
    let val = format!(r#"<script src="{}"></script>"#, src);
    self.scripts.push(val);
    self
  }

  /// Add a non-blocking `<link as="style">` tag. This is ideal for including
  /// styles that aren't essential for an initial render pass.
  ///
  /// Generally this should be combined with `.inline_style()` to optimize a
  /// render pipeline.
  // TODO: also allow passing a sha512
  pub fn style(mut self, src: &str) -> Self {
    let val = format!(r#"<link rel="preload" as="style" href="{}" onload="this.rel='stylesheet'">"#, src);
    self.styles.push(val);
    self
  }

  /// Add an inline `<style>` tag. This is ideal for including styles that
  /// should be available for an initial render pass.
  ///
  /// Generally this should be combined with `.style()` to optimize a render
  /// pipeline.
  // TODO: also allow passing a sha512
  pub fn inline_style(mut self, src: &str) -> Self {
    let val = format!(r#"<style>{}</style>"#, src);
    self.styles.push(val);
    self
  }

  /// Add a blocking `<link rel="stylesheet">` tag. This is ideal for externally
  /// loading scripts that should be loaded before any rendering can be
  /// initialized.
  // TODO: also allow passing a sha512
  pub fn blocking_style(mut self, src: &str) -> Self {
    let val = format!(r#"<link rel="stylesheet" href="{}">"#, src);
    self.styles.push(val);
    self
  }

  /// Add a favicon.
  pub fn favicon(mut self, src: &str) -> Self {
    let val =
      format!(r#"<link rel="icon" type="image/x-icon" href="{}">"#, src);
    self.favicon = Some(val);
    self
  }

  /// Add a `manifest.json` link.
  pub fn manifest(mut self, src: &str) -> Self {
    let val = format!(r#"<link rel="manifest" href="{}">"#, src);
    self.manifest = Some(val);
    self
  }

  /// Add a `<link as="font">` tag.
  pub fn font(mut self, src: &str) -> Self {
    let val = format!(
      r#"<link rel="preload" as="font" crossorigin href="{}">"#,
      src
    );
    self.fonts.push(val);
    self
  }

  /// Create an HTML document.
  pub fn build(self) -> String {
    let mut html: String = DOCTYPE.into();
    html.push_str(HTML_OPEN);
    html.push_str(HEAD_OPEN);
    html.push_str(CHARSET);
    html.push_str(VIEWPORT);
    for style in self.styles {
      html.push_str(&style);
    }
    for script in self.scripts {
      html.push_str(&script);
    }
    for font in self.fonts {
      html.push_str(&font);
    }
    if let Some(manifest) = self.manifest {
      html.push_str(&manifest);
    }
    if let Some(favicon) = self.favicon {
      html.push_str(&favicon);
    }
    html.push_str(HEAD_CLOSE);
    if let Some(body) = self.body {
      html.push_str(&body);
    }
    html.push_str(HTML_CLOSE);
    html
  }
}