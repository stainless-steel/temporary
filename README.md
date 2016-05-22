# Temporary [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides means of managing temporary files and directories.

## [Documentation][doc]

## Example

```rust
use std::fs::File;
use std::io::Write;
use temporary::Directory;

// Create a temporary directory.
let directory = Directory::new("foo").unwrap();

// Do some work.
let mut file = File::create(directory.join("foo.txt")).unwrap();
file.write_all(b"Hello, there!").unwrap();

// The directory and its content get removed automatically.
```

## Acknowledgments

The package was originally based on `std::io::TempDir` by Rust’s developers,
which was later moved to a separate crate,
[tempdir](https://github.com/rust-lang/tempdir).

## Contribution

Your contribution is highly appreciated. Do not hesitate to open an issue or a
pull request. Note that any contribution submitted for inclusion in the project
will be licensed according to the terms given in [LICENSE.md](LICENSE.md).

[doc]: https://stainless-steel.github.io/temporary
[status-img]: https://travis-ci.org/stainless-steel/temporary.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/temporary
[version-img]: https://img.shields.io/crates/v/temporary.svg
[version-url]: https://crates.io/crates/temporary
