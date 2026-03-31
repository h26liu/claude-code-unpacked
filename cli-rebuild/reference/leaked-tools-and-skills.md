# Leaked tools and skill-related prompts

This file summarizes the strongest public evidence for Claude Code's leaked tool and skill surface.

It is based primarily on:

- Anthropic Claude Code docs: <https://docs.anthropic.com/en/docs/claude-code>
- Piebald extracted prompt catalog: <https://github.com/Piebald-AI/claude-code-system-prompts>

## What is well-supported

Piebald's catalog explicitly lists built-in tool descriptions extracted from recent Claude Code releases. As of the repository snapshot describing Claude Code `v2.1.37` on February 7, 2026, the built-in tool descriptions include:

- `AskUserQuestion`
- `Bash`
- `Computer`
- `Edit`
- `EnterPlanMode`
- `ExitPlanMode`
- `Glob`
- `Grep`
- `LSP`
- `NotebookEdit`
- `ReadFile`
- `SendMessageTool`
- `Skill`
- `Sleep`
- `TaskCreate`
- `Task`
- `TeamDelete`
- `TeammateTool`
- `TodoWrite`
- `ToolSearch`
- `ToolSearch extended`
- `WebFetch`
- `WebSearch`
- `Write`

In practice, that means the leaked/public tool surface is much broader than the minimal local simulator originally exposed. The simulator now includes file-based modules for the major leaked tool categories so users can see and modify them directly.

That list is stronger than older community summaries because it is tied to a maintained extraction repo that tracks release-by-release changes.

## What is likely skill-related

The public evidence for `skills` is thinner than the evidence for tools. Anthropic's public docs foreground memories, hooks, slash commands, MCP, and subagents. Piebald's extracted prompt inventory, however, clearly names skill-related prompts:

- `Remember skill`
- `Skillify Current Session`
- `Skill` tool description

There is also a `Claude guide agent`, which is not necessarily a skill, but looks like a reusable guidance capability rather than a normal user prompt.

So the safest claim is:

- tools are directly evidenced by extracted tool descriptions
- skills are evidenced indirectly through prompt names and a dedicated `Skill` tool
- the exact user-facing boundary between `skill`, `agent`, `utility`, and `workflow` is still partly inferred

## How `cli-rebuild` uses this

The simulator in [`../README.md`](../README.md) does not claim to reproduce Anthropic's exact runtime. Instead it uses the leaked/public inventory to create a learnable, modular model:

- tools become file-based modules under [`../modules/tools/`](../modules/tools/)
- skill-like capabilities become modules under [`../modules/skills/`](../modules/skills/)
- workflows show how those pieces might be composed into multi-agent behavior

That makes the repo useful as both documentation and a playground.
