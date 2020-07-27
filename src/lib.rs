#![cfg_attr(test, deny(warnings))]
#![forbid(unsafe_code, missing_debug_implementations)]

//! Over the years the HTML spec has added lots of new capabilities in a backwards
//! compatible fashion. This means that even if something from the 90's might still
//! work in today's browsers, it might not always be the most efficient.
//!
//! This crate makes it easy to build performant HTML without needing to remember
//! all boilerplate involved.
//!
//! ## Examples
//!
//! ```rust
//! let res = html_index::Builder::new()
//!     .raw_body("<body>hello world</body>")
//!     .script("/bundle.js")
//!     .style("/bundle.css")
//!     .build();
//! println!("{}", res);
//! ```
//!
//! Which generates:
//!
//! ```html
//! <!DOCTYPE html>
//! <html>
//!   <head>
//!     <meta charset="utf-8">
//!     <meta name="viewport" content="width=device-width, initial-scale=1.0">
//!     <link rel="preload" as="style" href="/bundle.css" onload="this.rel='stylesheet'">
//!     <script src="/bundle.js" defer></script>
//!   </head>
//!   <body>hello world</body>
//! </html>
//! ```

const DOCTYPE: &str = "<!DOCTYPE html>";
const CHARSET: &str = r#"<meta charset="utf-8">"#;
const VIEWPORT: &str = r#"<meta name="viewport" content="width=device-width, initial-scale=1.0">"#;
const HTML_CLOSE: &str = "</html>";
const HEAD_OPEN: &str = "<head>";
const HEAD_CLOSE: &str = "</head>";

use std::default::Default;

/// Create a new `Builder` instance.
pub fn new<'a>() -> Builder<'a> {
    Builder::new()
}

/// Create a new HTML builder.
#[derive(Debug, Clone, Default)]
pub struct Builder<'b> {
    color: Option<String>,
    desc: Option<String>,
    lang: &'b str,
    favicon: Option<String>,
    fonts: Vec<String>,
    manifest: Option<String>,
    scripts: Vec<String>,
    styles: Vec<String>,
    title: Option<String>,
    body: Option<&'b str>,
    has_async_style: bool,
}

impl<'b> Builder<'b> {
    /// Create a new instance from an HTML body, including `<body></body>` tags.
    pub fn new() -> Self {
        Self {
            lang: "en-US",
            ..Default::default()
        }
    }

    /// Add a body to the document. The body must include `<body></body>` tags.
    pub fn raw_body(mut self, body: &'b str) -> Self {
        self.body = Some(body);
        self
    }

    /// Set the document language.
    pub fn lang(mut self, lang: &'b str) -> Self {
        self.lang = lang;
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

    /// Add a `<script></script>` tag. This is ideal for loading custom scripts
    /// that are essential for loading.
    // TODO: also allow passing a sha512
    pub fn inline_script(mut self, src: &str) -> Self {
        let val = format!(r#"<script>{}</script>"#, src);
        self.scripts.push(val);
        self
    }

    /// Add a non-blocking `<link as="style">` tag. This is ideal for including
    /// styles that aren't essential for an initial render pass.
    ///
    /// Generally this should be combined with `.inline_style()` to optimize a
    /// render pipeline.
    ///
    /// `onerror` exists because of a bug in firefox. See https://github.com/filamentgroup/loadCSS/issues/246 for more details
    // TODO: also allow passing a sha512
    pub fn style(mut self, src: &str) -> Self {
        let val = format!(
            r#"<link rel="preload" as="style" href="{}" onload="this.rel='stylesheet'" onerror="this.rel='stylesheet'">"#,
            src
        );
        self.styles.push(val);

        if !self.has_async_style {
            self = self.inline_script(css_rel_preload::CSS_REL_PRELOAD);
            self.has_async_style = true;
        }

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
        let val = format!(r#"<link rel="icon" type="image/x-icon" href="{}">"#, src);
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
        html.push_str(&format!(r#"<html lang="{}">"#, self.lang));
        html.push_str(HEAD_OPEN);
        html.push_str(CHARSET);
        html.push_str(VIEWPORT);
        if let Some(title) = self.title {
            html.push_str(&title);
        }
        if let Some(desc) = self.desc {
            html.push_str(&desc);
        }

        for script in self.scripts {
            html.push_str(&script);
        }
        for style in self.styles {
            html.push_str(&style);
        }
        for font in self.fonts {
            html.push_str(&font);
        }
        if let Some(manifest) = self.manifest {
            html.push_str(&manifest);
        }

        if let Some(color) = self.color {
            html.push_str(&color);
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

impl Into<http_types::Response> for Builder<'_> {
    fn into(self) -> http_types::Response {
        let mut res = http_types::Response::new(200);
        res.set_content_type(http_types::mime::HTML);
        res.set_body(self.build());
        res
    }
}
