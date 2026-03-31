# Hooks

Hooks are officially documented and make Claude Code feel much more like an automation runtime than a plain chat tool.

Anthropic documents hook events including:

- `PreToolUse`
- `PostToolUse`
- `Notification`
- `UserPromptSubmit`
- `Stop`
- `SubagentStop`
- `PreCompact`
- `SessionStart`
- `SessionEnd`

That event list is one of the strongest clues about Claude Code's runtime shape. It tells you the product is tracking:

- tool lifecycle
- user prompt lifecycle
- subagent lifecycle
- session lifecycle
- compaction lifecycle

The extracted prompt catalogs also include utility prompts related to hook handling, such as an `Agent Hook` prompt and a `Hook condition evaluator`, which makes the overall architecture feel even more event-driven.
