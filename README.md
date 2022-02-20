# Temporary [![Package][package-img]][package-url] [![Documentation][documentation-img]][documentation-url] [![Build][build-img]][build-url]

The package provides means of managing temporary files and directories.

## Example

```rust
use std::fs::File;
use std::io::Write;
use temporary::Directory;

// Create a temporary directory.
let directory = Directory::new("foo").unwrap();

// Do some work.
let mut file = File::create(directory.join("foo.txt")).unwrap();
file.write_all(b"Hi there!").unwrap();

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

[build-img]: https://github.com/stainless-steel/temporary/workflows/build/badge.svg
[build-url]: https://github.com/stainless-steel/temporary/actions/workflows/build.yml
[documentation-img]: https://docs.rs/temporary/badge.svg
[documentation-url]: https://docs.rs/temporary
[package-img]: https://img.shields.io/crates/v/temporary.svg
[package-url]: https://crates.io/crates/temporary
