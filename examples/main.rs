extern crate html_index;

pub fn main() {
  let res = html_index::Builder::new()
    .raw_body("<body>hello world</body>")
    .script("/bundle.js")
    .style("/bundle.css")
    .build();
  println!("{}", res);
}
