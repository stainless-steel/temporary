//! Temporary files and directories.
//!
//! ## Example
//!
//! ```rust
//! use std::fs::File;
//! use std::io::Write;
//! use temporary::Directory;
//!
//! // Create a temporary directory.
//! let root = Directory::new("foo").unwrap();
//!
//! // Do some work.
//! let mut file = File::create(root.join("foo.txt")).unwrap();
//! file.write_all(b"Hello, there!").unwrap();
//!
//! // The directory and its content get disposed automatically.
//! ```

use std::io::{Error, ErrorKind, Result};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{env, fs};

/// A temporary directory.
pub struct Directory {
    path: PathBuf,
    removed: bool,
}

impl Directory {
    /// Create a temporary directory.
    ///
    /// The directory will have a name starting from `prefix`, and it will be
    /// automatically disposed when the object goes out of scope.
    #[inline]
    pub fn new(prefix: &str) -> Result<Directory> {
        Directory::new_in(env::temp_dir(), prefix)
    }

    /// Create a temporary directory in a specific directory.
    ///
    /// The directory will have a name starting from `prefix`, and it will be
    /// automatically disposed when the object goes out of scope.
    pub fn new_in<T: AsRef<Path>>(root: T, prefix: &str) -> Result<Directory> {
        const RETRIES: u32 = 1 << 31;
        const CHARS: usize = 12;

        let root = root.as_ref();
        if !root.is_absolute() {
            let current = try!(env::current_dir());
            return Directory::new_in(current.join(root), prefix);
        }

        let mut state = random_state(root, prefix);
        for _ in 0..RETRIES {
            let suffix: String = random_string(CHARS, &mut state);

            let path = if prefix.is_empty() {
                root.join(&suffix)
            } else {
                root.join(&format!("{}.{}", prefix, suffix))
            };

            match fs::create_dir(&path) {
                Ok(_) => return Ok(Directory {
                    path: path.to_path_buf(),
                    removed: false,
                }),
                Err(error) => match error.kind() {
                    ErrorKind::AlreadyExists => {},
                    _ => return Err(error),
                },
            }
        }

        Err(Error::new(ErrorKind::AlreadyExists, "failed to find a vacant name"))
    }

    /// Return the path to the directory.
    #[inline]
    pub fn path(&self) -> &Path {
        self.as_ref()
    }

    /// Return the path to the directory and dispose the object without removing
    /// the actual directory.
    #[inline]
    pub fn into_path(mut self) -> PathBuf {
        self.removed = true;
        self.path.clone()
    }

    /// Remove the directory.
    #[inline]
    pub fn remove(mut self) -> Result<()> {
        self.cleanup()
    }

    fn cleanup(&mut self) -> Result<()> {
        if self.removed {
            return Ok(());
        }
        self.removed = true;

        fs::remove_dir_all(&self.path)
    }
}

impl AsRef<Path> for Directory {
    #[inline]
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl Deref for Directory {
    type Target = Path;

    #[inline]
    fn deref(&self) -> &Path {
        &self.path
    }
}

impl Drop for Directory {
    #[allow(unused_must_use)]
    #[inline]
    fn drop(&mut self) {
        self.cleanup();
    }
}

fn random_state(_: &Path, _: &str) -> [u64; 2] {
    use std::mem::uninitialized as rand;
    unsafe { [rand::<u64>() ^ 0x12345678, rand::<u64>() ^ 0x87654321] }
}

fn random_string(length: usize, state: &mut [u64; 2]) -> String {
    unsafe { String::from_utf8_unchecked((0..length).map(|_| random_letter(state)).collect()) }
}

// https://en.wikipedia.org/wiki/Xorshift#Xorshift.2B
fn random_letter(state: &mut [u64; 2]) -> u8 {
    let (mut x, y) = (state[0], state[1]);
    let number = {
        state[0] = y;
        x = x ^ (x << 23);
        x = x ^ (x >> 17);
        x = x ^ y ^ (y >> 26);
        state[1] = x;
        x.wrapping_add(y)
    };
    b'a' + (number % 26) as u8
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::Directory;

    #[test]
    fn new() {
        use std::fs;

        let path = {
            let directory = Directory::new("foo").unwrap();
            assert!(fs::metadata(directory.path()).is_ok());
            directory.path().to_path_buf()
        };
        assert!(fs::metadata(path).is_err());
    }

    #[test]
    fn deref() {
        let directory = Directory::new("bar").unwrap();
        work(&directory);

        fn work(_: &Path) {
        }
    }
}
