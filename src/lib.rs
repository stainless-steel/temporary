//! Temporary files and directories.
//!
//! ## Example
//!
//! ```rust
//! use temporary::Directory;
//!
//! let directory = Directory::new("foo").unwrap();
//! assert!(std::fs::metadata(directory).is_ok()); // Exists? Yes!
//! ```

extern crate rand;

use std::{env, fs};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

/// A temporary directory.
pub struct Directory {
    path: PathBuf,
    removed: bool,
}

impl Directory {
    /// Create a temporary directory.
    ///
    /// The directory will have a name starting from `prefix`, and it will be
    /// automatically removed when the object is disposed.
    #[inline]
    pub fn new(prefix: &str) -> Result<Directory> {
        Directory::new_in(env::temp_dir(), prefix)
    }

    /// Create a temporary directory in a specific directory.
    ///
    /// The directory will have a name starting from `prefix`, and it will be
    /// automatically removed when the object is destroyed.
    pub fn new_in<T: AsRef<Path>>(root: T, prefix: &str) -> Result<Directory> {
        use rand::Rng;

        const RETRIES: u32 = 1 << 31;
        const CHARS: usize = 12;

        let root = root.as_ref();
        if !root.is_absolute() {
            let current = try!(env::current_dir());
            return Directory::new_in(current.join(root), prefix);
        }

        let mut generator = rand::thread_rng();

        for _ in 0..RETRIES {
            let suffix: String = generator.gen_ascii_chars().take(CHARS).collect();

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

impl Drop for Directory {
    #[allow(unused_must_use)]
    #[inline]
    fn drop(&mut self) {
        self.cleanup();
    }
}

impl AsRef<Path> for Directory {
    #[inline]
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

#[cfg(test)]
mod tests {
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
}
