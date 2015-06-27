# Temporary [![Version][version-img]][version-url] [![Status][status-img]][status-url]

The package helps to create temporary files and directories.

## [Documentation][docs]

## Example

```rust
use temporary::Directory;

let directory = Directory::new("foo").unwrap();
assert!(std::fs::metadata(&directory).is_ok()); // Exists? Yes!
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
