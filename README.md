# Temporary [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package provides means of managing temporary files and directories.

## [Documentation][docs]

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

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[version-img]: https://img.shields.io/crates/v/temporary.svg
[version-url]: https://crates.io/crates/temporary
[status-img]: https://travis-ci.org/stainless-steel/temporary.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/temporary
[docs]: https://stainless-steel.github.io/temporary
