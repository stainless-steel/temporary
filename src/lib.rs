//! Temporary files and directories.

#![feature(env, fs, io, path)]

extern crate rand;

use std::{env, fs};
use std::io::{Error, ErrorKind, Result};
use std::path::{Path, PathBuf};

pub struct Directory {
    path: PathBuf,
    removed: bool,
}

impl Directory {
    /// Create a temporary directory. The directory will have a name starting
    /// from `prefix`, and it will be automatically removed when the object is
    /// destroyed.
    #[inline]
    pub fn new(prefix: &str) -> Result<Directory> {
        Directory::new_in(&env::temp_dir(), prefix)
    }

    /// Create a temporary directory in the location specified by `root`. The
    /// directory will have a name starting from `prefix`, and it will be
    /// automatically removed when the object is destroyed.
    pub fn new_in(root: &Path, prefix: &str) -> Result<Directory> {
        use rand::Rng;

        const RETRIES: u32 = 1 << 31;
        const CHARS: usize = 12;

        if !root.is_absolute() {
            let current = try!(env::current_dir());
            return Directory::new_in(&current.join(root), prefix);
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
                    ErrorKind::PathAlreadyExists => {},
                    _ => return Err(error),
                },
            }
        }

        Err(Error::new(ErrorKind::PathAlreadyExists, "failed to find a vacant name", None))
    }

    /// Return the path to the directory.
    #[inline]
    pub fn path<'d>(&'d self) -> &'d Path {
        &self.path
    }

    /// Remove the directory.
    #[inline]
    pub fn remove(mut self) -> Result<()> {
        self.cleanup()
    }

    /// Dispose the object without removing the actual directory.
    #[inline]
    pub fn unwrap(mut self) -> Result<()> {
        self.removed = true;
        Ok(())
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
    #[inline]
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

#[cfg(test)]
mod tests {
    use super::Directory;

    #[test]
    fn new() {
        use std::fs::PathExt;

        let path = {
            let directory = Directory::new("foo").unwrap();
            assert!(directory.path().exists());
            directory.path().to_path_buf()
        };
        assert!(!(&path).exists());
    }
}
