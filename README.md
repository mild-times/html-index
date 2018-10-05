# html-index
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Generate an HTML index.

- [Documentation][8]
- [Crates.io][2]
- [Releases][releases]

## Examples
### Basic
```rs
extern crate html_index;

use html_index;

fn main () {
  let index = html_index::generate(None, None);
  println!("html: ${}", index);
}
```

## Installation
```sh
$ cargo add html-index
```

## Safety
This crate uses ``#![deny(unsafe_code)]`` to ensure everything is implemented in
100% Safe Rust.

## Contributing
Want to join us? Check out our ["Contributing" guide][contributing] and take a
look at some of these issues:

- [Issues labeled "good first issue"][good-first-issue]
- [Issues labeled "help wanted"][help-wanted]

## References
None.

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/html-index.svg?style=flat-square
[2]: https://crates.io/crates/html-index
[3]: https://img.shields.io/travis/chooxide/html-index.svg?style=flat-square
[4]: https://travis-ci.org/chooxide/html-index
[5]: https://img.shields.io/crates/d/html-index.svg?style=flat-square
[6]: https://crates.io/crates/html-index
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/html-index

[releases]: https://github.com/chooxide/html-index/releases
[contributing]: https://github.com/chooxide/html-index/.github/CONTRIBUTING.md
[good-first-issue]: https://github.com/chooxide/html-index/labels/good%20first%20issue
[help-wanted]: https://github.com/chooxide/html-index/labels/help%20wanted
