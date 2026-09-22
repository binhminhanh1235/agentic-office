# Architecture

```text
Agents
  | stdio / HTTP
  v
MCP Transport Layer
  v
Tool Registry + Request Validation
  v
Policy / Capability / Audit Layer
  v
Document Service ---------------- Editor Bridge
  |                                      |
  +-- Markdown adapter                   +-- session discovery
  +-- DOCX adapter                       +-- state inspection
  +-- XLSX adapter                       +-- bounded operations
  v
Workspace and Revision Store
```

## Process boundaries

The initial gateway runs as a sidecar process. It must not depend on internal UI implementation details of the desktop editor. The editor bridge is an explicit adapter that can be unavailable without making local document operations unavailable.

## Read path

1. Validate workspace and document identifier.
2. Resolve the path inside an allowed workspace root.
3. Detect the supported format.
4. Load a consistent snapshot and compute a revision identity.
5. Return structured content plus capability metadata.

## Write path

1. Receive a proposal or direct write request.
2. Validate the requested operation against the document format and policy.
3. Verify the expected revision if one was supplied.
4. Produce a preview/change set.
5. Require explicit approval for direct mutation when policy demands it.
6. Write to a temporary file, validate the result, then atomically replace the target.
7. Emit an audit record containing actor, operation, document, old revision and new revision.

## Concurrency model

Every mutable document has a revision identity. A write that supplies `expected_revision` must fail with a conflict if the current revision differs. The gateway must never silently overwrite a newer version produced by another agent or by the editor.

## Editor bridge boundary

The bridge must expose capabilities rather than pretending all operations are supported. For example, `get_document_state` may be supported before `insert_text` or `replace_range`. Unsupported operations return a typed capability error and do not fall back to unsafe filesystem mutation.

## Security baseline

- Workspace roots are explicitly configured.
- Path traversal and symlink escape are rejected.
- No arbitrary shell execution is exposed through MCP tools.
- HTTP transport requires authentication before remote use.
- Audit logging is enabled for all write and editor operations.
