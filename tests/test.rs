extern crate failure;
extern crate html_index;

use failure::Error;

#[test]
fn should_work() -> Result<(), Error> {
  let res = html_index::Builder::new()
    .raw_body("<body>hello world</body>")
    .script("/bundle.js")
    .style("/bundle.css")
    .build();

  let expected = include_str!("./fixtures/html-css.html");
  assert_eq!(res, expected.replace("\n", ""));
  Ok(())
}
