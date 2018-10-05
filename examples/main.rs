extern crate html_index;


pub fn main () {
  let body = "<body>hello world</body>";
 let res = html_index::Builder::new(body).build();
 println!("{:?}", res);
}
