# Implementation plan

## Milestone 0: foundation (current)

- [x] Isolate MCP in `agentic-mcp/`.
- [x] Add Rust executable manifest.
- [x] Add transport/workspace configuration entrypoint.
- [x] Document architecture, safety and concurrency boundaries.
- [ ] Add CI job for formatting, linting and tests.

## Milestone 1: domain and policy

- [ ] `DocumentId`, `WorkspaceId`, `RevisionId` and format capability types.
- [ ] Workspace root resolver with traversal and symlink protection.
- [ ] Read-only metadata and extraction service traits.
- [ ] Write intent, proposal, preview and audit models.
- [ ] Typed errors for unsupported format, policy denial and revision conflict.

## Milestone 2: Markdown backend

- [ ] Metadata and UTF-8 extraction.
- [ ] Heading outline and text search.
- [ ] Atomic writes with expected revision checks.
- [ ] Proposal-to-apply flow with fixture tests.

## Milestone 3: DOCX and XLSX adapters

- [ ] DOCX paragraphs, headings, tables and safe text-level operations.
- [ ] XLSX workbook/sheet metadata, cell ranges and formula-preserving read path.
- [ ] Explicit capability matrix for unsupported structural operations.
- [ ] Golden fixtures and round-trip validation.

## Milestone 4: transports

- [ ] MCP stdio protocol handler.
- [ ] HTTP/streamable HTTP handler.
- [ ] Authentication and request correlation for HTTP.
- [ ] Contract tests shared by both transports.

## Milestone 5: editor bridge

- [ ] Session discovery adapter.
- [ ] Read-only editor state inspection.
- [ ] Capability negotiation.
- [ ] Bounded mutation operations with editor-side acknowledgement.

## Definition of done

A milestone is complete only when it includes tests, failure-path behavior, documentation and a direct verification against the repository branch. A tool must not advertise an operation before its backend and policy checks are implemented.
