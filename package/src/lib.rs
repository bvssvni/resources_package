//! Defines the `Package` struct that is generated by the `resources_package!` macro.

use std::path::{Path, PathBuf};

/// Represents a package of resources created by `resources_package!`.
#[allow(missing_copy_implementations)]
pub struct Package {
    /// This member is public for technical reasons. Don't use it directly.
    pub data: &'static [(&'static [u8], &'static [u8])]
}

pub struct Iter<'a>(std::slice::Iter<'a, (&'static [u8], &'static [u8])>);

impl Package {
    /// Attempts to find a resource in the package.
    ///
    /// The parameter is a path relative to the directory specified in the macro. For example if
    ///  if you call `resources_package!("../resources")`, calling `find` with `images/file.png`
    ///  will attempt to find the file that was at `../resources/images/file.png` at compile-time.
    ///
    /// Returns the content of the file, or `None` if it was not found.
    ///
    /// If multiple resources have the same path, the first one will be returned. You should try
    ///  to avoid this situation.
    pub fn find<P>(&self, resource: P) -> Option<&'static [u8]> where P: AsRef<Path> {
        let resource = resource.as_ref();
        self.data.iter().find(|&&(path, _)| {
            Path::new(&String::from_utf8(path.to_vec()).unwrap()) == resource
        }).map(|v| v.1)
    }

    /// Returns an iterator to the entries in the package.
    ///
    /// The return type implements `Iterator<(Path, &'static [u8])>`.
    pub fn iter<'a>(&'a self) -> Iter<'a> {
        Iter(self.data.iter())
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (PathBuf, &'static [u8]);
    fn next(&mut self) -> Option<(PathBuf, &'static [u8])> {
        match self.0.next() {
            None => None,
            Some(&(path, content)) => {
                Some((PathBuf::from(&String::from_utf8(path.to_vec()).unwrap()), content))
            }
        }
    }
}
