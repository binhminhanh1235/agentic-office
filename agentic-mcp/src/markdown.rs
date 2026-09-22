use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};
use tokio::fs;

use crate::domain::DocumentRevision;
use crate::policy::resolve_safe_path;
use crate::domain::PolicyError;

#[derive(Debug)]
pub enum MarkdownError {
    Policy(PolicyError),
    Io(std::io::Error),
}

impl From<PolicyError> for MarkdownError {
    fn from(value: PolicyError) -> Self {
        Self::Policy(value)
    }
}

impl From<std::io::Error> for MarkdownError {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

pub async fn read(workspace: &Path, relative_path: &Path) -> Result<(String, DocumentRevision), MarkdownError> {
    let path = resolve_safe_path(workspace, relative_path)?;
    let bytes = fs::read(path).await?;
    let revision = revision_for(&bytes);
    let content = String::from_utf8_lossy(&bytes).into_owned();
    Ok((content, revision))
}

pub async fn write_atomic(
    workspace: &Path,
    relative_path: &Path,
    expected: &DocumentRevision,
    content: &str,
) -> Result<DocumentRevision, MarkdownError> {
    let path = resolve_safe_path(workspace, relative_path)?;
    let current = fs::read(&path).await?;
    let current_revision = revision_for(&current);
    if &current_revision != expected {
        return Err(MarkdownError::Policy(PolicyError::RevisionMismatch));
    }

    let temporary = temporary_path(&path);
    fs::write(&temporary, content.as_bytes()).await?;
    fs::rename(&temporary, &path).await?;
    Ok(revision_for(content.as_bytes()))
}

fn revision_for(bytes: &[u8]) -> DocumentRevision {
    let digest = Sha256::digest(bytes);
    DocumentRevision {
        sha256: format!("{digest:x}"),
        byte_len: bytes.len() as u64,
    }
}

fn temporary_path(path: &Path) -> PathBuf {
    let mut temporary = path.as_os_str().to_os_string();
    temporary.push(".agentic-office.tmp");
    PathBuf::from(temporary)
}
