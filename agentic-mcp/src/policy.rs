use std::path::{Component, Path, PathBuf};

use crate::domain::{DocumentFormat, DocumentRevision, PolicyError, WriteIntent, WriteMode};

pub fn resolve_safe_path(workspace: &Path, relative: &Path) -> Result<PathBuf, PolicyError> {
    if relative.is_absolute() {
        return Err(PolicyError::AbsolutePathNotAllowed);
    }

    if relative.components().any(|component| matches!(component, Component::ParentDir)) {
        return Err(PolicyError::PathTraversalNotAllowed);
    }

    Ok(workspace.join(relative))
}

pub fn detect_format(path: &Path) -> Result<DocumentFormat, PolicyError> {
    match path.extension().and_then(|value| value.to_str()) {
        Some("md") | Some("markdown") => Ok(DocumentFormat::Markdown),
        Some("docx") => Ok(DocumentFormat::Docx),
        Some("xlsx") => Ok(DocumentFormat::Xlsx),
        _ => Err(PolicyError::UnsupportedFormat),
    }
}

pub fn validate_write_intent(intent: &WriteIntent) -> Result<(), PolicyError> {
    if intent.mode == WriteMode::Direct && intent.expected_revision.is_none() {
        return Err(PolicyError::DirectWriteRequiresExpectedRevision);
    }
    Ok(())
}

pub fn revision_matches(expected: &DocumentRevision, actual: &DocumentRevision) -> Result<(), PolicyError> {
    if expected == actual {
        Ok(())
    } else {
        Err(PolicyError::RevisionMismatch)
    }
}
