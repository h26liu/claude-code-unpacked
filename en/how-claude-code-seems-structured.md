# How Claude Code seems to be structured

This document mixes two categories of evidence:

- official product documentation from Anthropic
- reasonable architectural inference from extracted prompt catalogs

Whenever this file says "appears," "seems," or "likely," that is an inference rather than an official product statement.

## The visible outer layer

From Anthropic's own docs, Claude Code has a clear user-facing extension model:

- `CLAUDE.md` files for persistent memory
- custom slash commands in `.claude/commands/`
- custom subagents in `.claude/agents/`
- hooks configured in settings JSON

## The likely inner layer

The extracted prompt inventories suggest Claude Code has a second, more internal layer composed of many prompt modules:

- a main system prompt
- builtin tool descriptions
- agent-specific prompts
- utility prompts
- runtime reminders
