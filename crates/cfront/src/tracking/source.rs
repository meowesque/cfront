use std::{cell::RefCell, rc::Rc};
use super::file::{FileManager, FileRef};

pub(crate) type Id = u64;

/// An opaque identifier related to a source file in the SourceManager.
pub struct SourceId(pub(crate) Id);

/// A position within a source file. Exchange this with the
/// SourceManager to retrieve information about the position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourcePos(pub(crate) Id);

/// A file entry, containing additional information for a file.
#[derive(Debug, Clone)]
pub(crate) struct FileEntry {
  file_ref: FileRef,
  include_pos: Option<SourcePos>,
}

/// An expansion entry, containing additional information for a macro expansion.
#[derive(Debug, Clone)]
pub(crate) struct ExpansionEntry {}

/// An entry in the SourceManager, representing either a file or an expansion.
#[derive(Debug, Clone)]
pub(crate) enum SourceEntry {
  File(FileEntry),
  Expansion(ExpansionEntry),
}

#[derive(Debug, Clone)]
pub struct SourceManager {
  file_manager: FileManager,

  /// Collection of source entries, indexed via their SourceId.
  source_entries: Rc<RefCell<Vec<SourceEntry>>>,

  /// Offset accumulated from the lengths of every source file.
  cumulative_offset: Rc<RefCell<Id>>,
}

impl SourceManager {
  pub fn new(file_manager: FileManager) -> Self {
    Self {
      file_manager,
      source_entries: Default::default(),
      cumulative_offset: Default::default(),
    }
  }

  /// Retrieve the FileManager of the SourceManager.
  pub fn file_manager(&self) -> FileManager {
    self.file_manager.clone()
  }

  /// Create a new SourceId from a file reference.
  pub fn create_source_id(&self, file_ref: FileRef, include_pos: Option<SourcePos>) -> SourceId {
    let mut source_entries = self.source_entries.borrow_mut();
    let mut cumulative_offset = self.cumulative_offset.borrow_mut();

    let source_id = SourceId(source_entries.len() as Id);

    source_entries.push(SourceEntry::File(FileEntry {
      file_ref: file_ref.clone(),
      include_pos,
    }));

    *cumulative_offset += file_ref.content().as_ref().len() as Id;

    source_id
  }
}
