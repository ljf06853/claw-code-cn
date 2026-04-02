# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Claw Code is a local coding-agent CLI implemented in safe Rust, inspired by Claude Code. The Rust workspace is the primary implementation surface. The Python `src/` and `tests/` trees support parity verification against the TypeScript original.

## Build, Test, and Verification

### Rust workspace (primary implementation)

```bash
cd rust/
cargo build
cargo build --release
cargo build --release -p claw-cli    # build only the CLI binary
```

### Verification (required before PR)

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo check --workspace
cargo test --workspace
```

### Run the CLI

```bash
cargo run --bin claw -- --help
cargo run --bin claw -- prompt "summarize this workspace"
```

Install locally:

```bash
cargo install --path rust/crates/claw-cli --locked
```

### Python workspace (parity verification)

```bash
python3 -m src.main summary          # render Python porting summary
python3 -m src.main manifest         # print Python workspace manifest
python3 -m unittest discover -s tests -v   # run Python verification tests
```

## Repository Structure

```
rust/
├── Cargo.toml                       # Workspace root; all crates live under crates/
├── crates/
│   ├── api/                         # Provider client with OAuth, SSE streaming
│   ├── runtime/                     # Session state, config, permissions, tools, MCP orchestration
│   ├── claw-cli/                    # CLI binary: REPL, rendering, init/bootstrap
│   ├── commands/                   # Slash command registry (/help, /status, /model, etc.)
│   ├── tools/                      # Built-in tool definitions (mvp_tool_specs)
│   ├── plugins/                     # Plugin discovery and hook pipeline
│   ├── lsp/                         # LSP client types and process helpers
│   ├── server/                      # HTTP/SSE server (axum)
│   └── compat-harness/              # Upstream editor integration compatibility
src/                                 # Python porting workspace (not runtime)
tests/                               # Python verification tests
```

## Architecture Notes

- **Workspace resolver 2** is enabled; all crate dependencies must be explicit.
- **`unsafe_code`** is forbidden across the workspace; use safe Rust only.
- **Runtime session flow**: `conversation.rs` holds the core agent loop; it calls into `tools/src/lib.rs` for tool dispatch and `api/src/client.rs` for LLM responses.
- **Tool registry**: `mvp_tool_specs()` in `crates/tools/src/lib.rs` defines current built-in tools (shell/file/search/web/todo/skill/agent/config/notebook/repl/powershell).
- **Slash commands**: registered in `crates/commands/src/lib.rs`; handled in `crates/claw-cli/src/main.rs`.
- **Config loading**: `CLAW.md` and `.claw.json` are discovered and merged by `crates/runtime/src/config.rs`.
- **MCP support**: `crates/runtime/src/mcp*.rs` handle MCP bootstrap, stdio, and client lifecycle.

## Important Parity Context

The Rust implementation is **not yet feature-complete** with the TypeScript original. See `PARITY.md` for the full gap analysis. Notable gaps:

- Hooks are parsed but not executed (config-only in Rust)
- Plugins are missing in Rust
- Many slash commands absent (`/agents`, `/hooks`, `/mcp`, `/plugin`, `/skills`, `/tasks`, etc.)
- Skills are local-file only without TS-style registry/bundled pipeline

## Python Workspace

The `src/` and `tests/` directories are Python porting verification surfaces, not runtime code. Commands like `python3 -m src.main summary` and `python3 -m unittest discover -s tests -v` drive parity audit workflows, not the actual CLI.