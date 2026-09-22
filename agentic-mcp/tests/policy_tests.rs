use std::path::Path;

use agentic_office_mcp::domain::{DocumentRevision, PolicyError, WriteIntent, WriteMode};
use agentic_office_mcp::policy::{detect_format, resolve_safe_path, revision_matches, validate_write_intent};

#[test]
fn rejects_absolute_paths() {
    let result = resolve_safe_path(Path::new("/workspace"), Path::new("/etc/passwd"));
    assert_eq!(result, Err(PolicyError::AbsolutePathNotAllowed));
}

#[test]
fn rejects_parent_directory_traversal() {
    let result = resolve_safe_path(Path::new("/workspace"), Path::new("../outside.md"));
    assert_eq!(result, Err(PolicyError::PathTraversalNotAllowed));
}

#[test]
fn detects_supported_formats() {
    assert!(detect_format(Path::new("notes.md")).is_ok());
    assert!(detect_format(Path::new("report.docx")).is_ok());
    assert!(detect_format(Path::new("budget.xlsx")).is_ok());
    assert!(detect_format(Path::new("image.png")).is_err());
}

#[test]
fn direct_write_requires_revision() {
    let intent = WriteIntent {
        document: crate::support_document(),
        expected_revision: None,
        mode: WriteMode::Direct,
        proposed_content: "updated".to_owned(),
    };

    assert_eq!(validate_write_intent(&intent), Err(PolicyError::DirectWriteRequiresExpectedRevision));
}

#[test]
fn detects_revision_conflict() {
    let expected = DocumentRevision { sha256: "a".into(), byte_len: 1 };
    let actual = DocumentRevision { sha256: "b".into(), byte_len: 1 };
    assert_eq!(revision_matches(&expected, &actual), Err(PolicyError::RevisionMismatch));
}

mod support {
    use std::path::PathBuf;
    use agentic_office_mcp::domain::{DocumentFormat, DocumentRef};

    pub fn document() -> DocumentRef {
        DocumentRef { relative_path: PathBuf::from("notes.md"), format: DocumentFormat::Markdown }
    }
}

fn support_document() -> agentic_office_mcp::domain::DocumentRef {
    support::document()
}
