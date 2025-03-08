use crate::prelude::*;
use std::{
  cell::RefCell,
  collections::HashMap,
  path::{Path, PathBuf},
  rc::Rc,
};

#[derive(Debug, Clone)]
pub struct Options {
  /// If set, paths are resolved as if they were relative to this directory.
  pub working_directory: Option<PathBuf>,
}

/// A cheaply cloneable reference to a file in the file manager.
#[derive(Debug, Clone)]
pub struct FileRef {
  path: Rc<PathBuf>,
  content: Rc<String>,
}

impl FileRef {
  /// Retrieve the path of the file.
  pub fn path<'a>(&'a self) -> impl AsRef<Path> + use<'a> {
    self.path.as_ref()
  }

  /// Retrieve the content of the file.
  pub fn content<'a>(&'a self) -> impl AsRef<str> + use<'a> {
    self.content.as_ref()
  }
}

/// Manager for loading files from disk and caching them in memory.
#[derive(Debug, Clone)]
pub struct FileManager {
  options: Rc<Options>,
  cache: Rc<RefCell<HashMap<PathBuf, FileRef>>>,
}

impl FileManager {
  pub fn new(options: Options) -> Self {
    Self {
      options: Rc::new(options),
      cache: Default::default(),
    }
  }

  /// Retrieve the options of the file manager.
  pub fn options(&self) -> &Options {
    &self.options
  }

  /// Retrieve a file from the cache or load it from disk.
  /// Currently, files are assumed to be plaintext by default.
  pub fn get_file(&self, path: impl AsRef<Path>) -> Result<FileRef> {
    let mut cache = self.cache.borrow_mut();

    match cache.get(path.as_ref()) {
      Some(file) => Ok(file.clone()),
      None => {
        let path = match &self.options.working_directory {
          // Only resolve relative paths.
          Some(working_directory) if path.as_ref().is_relative() => {
            working_directory.join(path.as_ref())
          }
          _ => path.as_ref().to_path_buf(),
        };

        let content = std::fs::read_to_string(path.clone())?;

        let file_ref = FileRef {
          path: Rc::new(path.clone()),
          content: Rc::new(content),
        };

        cache.insert(path.clone(), file_ref.clone());

        Ok(file_ref)
      }
    }
  }
}
