# Temporary [![Build Status][status-img]][status-url]

The library facilitates the creation of temporary files and directories.

## [Documentation][1]

## Example

```rust
use temporary::Directory;

let directory = Directory::new("foo").unwrap();
assert!(std::fs::metadata(directory).is_ok()); // Exists? Yes!
```

## Acknowledgments

The package was originally based on `std::io::TempDir` by Rust’s developers,
which was later moved to a separate crate,
[tempdir](https://github.com/rust-lang/tempdir).

## Contribution

1. Fork the project.
2. Implement your idea.
3. Open a pull request.

[1]: https://stainless-steel.github.io/temporary

[status-img]: https://travis-ci.org/stainless-steel/temporary.svg?branch=master
[status-url]: https://travis-ci.org/stainless-steel/temporary
