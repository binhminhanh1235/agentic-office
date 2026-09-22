use std::path::PathBuf;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentFormat {
    Markdown,
    Docx,
    Xlsx,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteMode {
    Proposal,
    Direct,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentRef {
    pub relative_path: PathBuf,
    pub format: DocumentFormat,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentRevision {
    pub sha256: String,
    pub byte_len: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WriteIntent {
    pub document: DocumentRef,
    pub expected_revision: Option<DocumentRevision>,
    pub mode: WriteMode,
    pub proposed_content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyError {
    AbsolutePathNotAllowed,
    PathTraversalNotAllowed,
    UnsupportedFormat,
    DirectWriteRequiresExpectedRevision,
    RevisionMismatch,
}
