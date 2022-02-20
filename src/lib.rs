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
//! let directory = Directory::new("foo").unwrap();
//!
//! // Do some work.
//! let mut file = File::create(directory.join("foo.txt")).unwrap();
//! file.write_all(b"Hello, there!").unwrap();
//!
//! // The directory and its content get removed automatically.
//! ```

extern crate random;

use random::Source;
use std::io::{Error, ErrorKind, Result};
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::{env, fmt, fs};

/// A temporary directory.
pub struct Directory {
    path: PathBuf,
    removed: bool,
}

impl Directory {
    /// Create a temporary directory.
    ///
    /// The directory will have a name starting from `prefix`, and it will be
    /// automatically removed when the object goes out of scope.
    #[inline]
    pub fn new(prefix: &str) -> Result<Directory> {
        Directory::with_parent(env::temp_dir(), prefix)
    }

    /// Create a temporary directory in a specific directory.
    ///
    /// The directory will have a name starting from `prefix`, and it will be
    /// automatically removed when the object goes out of scope.
    pub fn with_parent<T: AsRef<Path>>(parent: T, prefix: &str) -> Result<Directory> {
        const RETRIES: u32 = 1 << 31;
        const CHARS: usize = 12;

        let parent = parent.as_ref();
        if !parent.is_absolute() {
            let current = env::current_dir()?;
            return Directory::with_parent(current.join(parent), prefix);
        }

        let mut source = random::default().seed(random_seed(parent, prefix));
        for _ in 0..RETRIES {
            let suffix: String = random_string(CHARS, &mut source);

            let path = if prefix.is_empty() {
                parent.join(&suffix)
            } else {
                parent.join(&format!("{}.{}", prefix, suffix))
            };

            match fs::create_dir(&path) {
                Ok(_) => {
                    return Ok(Directory {
                        path: path.to_path_buf(),
                        removed: false,
                    })
                }
                Err(error) => match error.kind() {
                    ErrorKind::AlreadyExists => {}
                    _ => return Err(error),
                },
            }
        }

        Err(Error::new(
            ErrorKind::AlreadyExists,
            "failed to find a vacant name",
        ))
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

impl fmt::Debug for Directory {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.path.fmt(formatter)
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

fn random_seed(_: &Path, _: &str) -> [u64; 2] {
    use std::mem::uninitialized as rand;
    unsafe { [rand::<u64>() ^ 0x12345678, rand::<u64>() ^ 0x87654321] }
}

fn random_string<S: Source>(length: usize, source: &mut S) -> String {
    unsafe { String::from_utf8_unchecked((0..length).map(|_| random_letter(source)).collect()) }
}

fn random_letter<S: Source>(source: &mut S) -> u8 {
    b'a' + (source.read::<u64>() % 26) as u8
}

#[cfg(test)]
mod tests {
    use super::Directory;
    use std::path::Path;

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

        fn work(_: &Path) {}
    }
}
