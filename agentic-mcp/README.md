# Agentic Office MCP

MCP gateway for Codex, Claude Code, Antigravity and other compatible agents to inspect and interact with documents through a controlled service boundary.

## Initial configuration

- **Transport:** stdio and HTTP
- **Scope:** hybrid local document access plus editor bridge
- **Formats:** Markdown, DOCX, XLSX
- **Write policy:** mixed proposal/dry-run and direct write
- **Integration:** hybrid sidecar plus editor integration

## Design rules

1. The MCP gateway is isolated from ONLYOFFICE submodules.
2. Read operations are capability-scoped and path-sandboxed.
3. Write operations must expose intent, affected revision, and a previewable change set.
4. Direct writes require an explicit policy decision and optimistic concurrency checks.
5. Editor integration is represented by a bridge contract; unsupported UI actions must return a typed capability error.
6. File mutation is atomic and must preserve the original file when validation fails.

## Planned tool groups

### Discovery and read

- `workspace.list`
- `document.get_metadata`
- `document.extract`
- `document.outline`
- `document.search`
- `document.get_revision`

### Proposal and write

- `document.propose_change`
- `document.preview_change`
- `document.apply_change`
- `document.rollback`

### Editor bridge

- `editor.list_sessions`
- `editor.get_document_state`
- `editor.request_operation`

The first implementation milestone establishes the domain contracts and service boundaries. Format adapters and concrete MCP protocol handlers are added incrementally behind these contracts.
