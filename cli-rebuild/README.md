# Claude Code clean-room CLI rebuild

This directory contains a working Rust CLI runtime that follows a Claude Code-like structure: layered instructions, memory files, reminders, tools, agents, workflows, commands, hooks, and delegated sub-tasks.

It is a clean-room reconstruction, not Anthropic's internal source tree. The goal is not to claim hidden parity with Claude Code's internal reasoning. The goal is to make the surrounding runtime legible and hackable so users can experiment with the structure using any model provider they choose.

## What is implemented

- a Rust CLI binary under [`Cargo.toml`](Cargo.toml) and [`rust-src/`](rust-src)
- provider and model selection through `cli-rebuild.config.toml`, `.env`, and environment variables
- support for `mock`, `anthropic`, `openai`, `openrouter`, and generic `openai-compatible` APIs
- a prompt-driven agent loop with JSON tool-call envelopes
- Claude Code-like prompt layers for identity, execution, tooling, delegation, memory, and commands/hooks
- executable local tools such as `ReadFile`, `Write`, `Edit`, `Glob`, `Grep`, `Bash`, `LSP`, `TodoWrite`, `AskUserQuestion`, `Skill`, `ToolSearch`, `WebFetch`, and `WebSearch`
- delegated `Task` execution to subagents like `Explore`, `Plan`, `Task`, `Review`, and `Swarm coordinator`
- command and hook concepts in the runtime catalog
- project and user memory loading from files like `CLAUDE.md` and `.claude/CLAUDE.md`
- inspect and trace commands for learning how the runtime assembles context

## Structure

- [`Cargo.toml`](Cargo.toml): Rust crate manifest
- [`rust-src/main.rs`](rust-src/main.rs): CLI entrypoint
- [`rust-src/config.rs`](rust-src/config.rs): config, env, and hook loading
- [`rust-src/providers.rs`](rust-src/providers.rs): provider adapters
- [`rust-src/protocol.rs`](rust-src/protocol.rs): model response contract
- [`rust-src/tools.rs`](rust-src/tools.rs): executable tools and catalog surface
- [`rust-src/session.rs`](rust-src/session.rs): session assembly, memory loading, and local Claude file discovery
- [`rust-src/agent_loop.rs`](rust-src/agent_loop.rs): agent loop, hook application, and subagent delegation
- [`rust-src/catalog.rs`](rust-src/catalog.rs): reconstructed layers, workflows, agents, commands, and hooks
- [`cli-rebuild.config.toml.example`](cli-rebuild.config.toml.example): example settings
- [`.env.example`](.env.example): example environment variables

The older JS files are still in the directory as reference material from the earlier simulator. The Rust crate is the active runtime.

## Quick start

```bash
cd cli-rebuild
cp cli-rebuild.config.toml.example cli-rebuild.config.toml
cp .env.example .env
cargo run -- inspect
cargo run -- config
cargo run -- trace --task "review the repo structure and explain the runtime" --scenario review --plan-mode
cargo run -- run --task "inspect this repo and explain its structure"
cargo run -- chat
```

## Configure a real model

Set the provider in `cli-rebuild.config.toml` or `.env`.

Anthropic example:

```toml
[provider]
type = "anthropic"
model = "claude-sonnet-4-5"
api_key_env = "CLI_REBUILD_API_KEY"
```

OpenAI example:

```toml
[provider]
type = "openai"
model = "gpt-5"
api_key_env = "CLI_REBUILD_API_KEY"
```

OpenRouter example:

```toml
[provider]
type = "openrouter"
model = "anthropic/claude-sonnet-4.5"
api_key_env = "CLI_REBUILD_API_KEY"
```

Generic OpenAI-compatible example:

```toml
[provider]
type = "openai-compatible"
model = "your-model-name"
base_url = "https://your-endpoint.example/v1"
api_key_env = "CLI_REBUILD_API_KEY"
```

Then set the key in `.env`:

```bash
CLI_REBUILD_API_KEY=your_key_here
```

## Claude-style local context

The runtime looks for local project or user memory files and folds them into the assembled session. Right now it checks these common paths:

- `./CLAUDE.md`
- `./.claude/CLAUDE.md`
- `./.claude/memory.md`
- `~/.claude/CLAUDE.md`
- `~/.config/claude-code/CLAUDE.md`
- `~/.config/cli-rebuild/CLAUDE.md`

It also scans for local Claude-style extension files in:

- `./.claude/commands/`
- `./.claude/agents/`
- `./.claude/prompts/`

Those files are now parsed into local command, agent, and prompt objects, then injected into the assembled session prompt. A small sample pack is included under `cli-rebuild/.claude/` so the behavior is visible immediately.


## Included sample pack

`cli-rebuild/.claude/` now ships with a few example files so users can see local runtime extension points without creating anything first:

- `commands/review.md`
- `commands/architecture.md`
- `agents/doc-guide.md`
- `agents/tool-auditor.md`
- `prompts/teaching.md`
- `prompts/clean-room.md`
- `CLAUDE.md`

## Commands

- `cargo run -- inspect`
  Shows the reconstructed runtime catalog: layers, agents, skills, commands, hooks, tools, and workflows.
- `cargo run -- config`
  Prints the effective config with secrets redacted.
- `cargo run -- trace --task "..." --scenario planning --plan-mode`
  Shows how the runtime would assemble context and route work.
- `cargo run -- run --task "..."`
  Runs a real provider-backed task loop.
- `cargo run -- chat`
  Starts an interactive CLI session.

## Notes

This runtime exposes the structure around the reasoning loop, not the hidden reasoning itself. It is useful for understanding and remixing:

- prompt layering
- tool manifests
- memory injection
- delegated subagents
- workflow routing
- hook points around tool execution and final output

The next natural steps are:

- loading commands, prompts, and agents from disk instead of only discovering them
- making the tool registry and workflow catalog file-driven
- adding a richer TUI in Rust
- expanding provider support and response adapters
- pushing the runtime closer to a full Claude Code-like terminal product
