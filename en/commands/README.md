# Commands

Anthropic documents two command layers:

- builtin slash commands such as `/agents`, `/memory`, `/review`, `/mcp`, and `/compact`
- custom slash commands stored as markdown files in `.claude/commands/`

That already makes commands more interesting than a normal CLI help menu. Claude Code commands are partly product features and partly file-backed prompts.

The extracted prompt inventory reinforces that. Piebald's catalog includes dedicated prompts for command-like flows such as:

- `/pr-comments`
- `/review-pr`
- `/security-review`

That suggests commands are not just thin wrappers over generic chat behavior. At least some of them appear to carry their own specialized prompt packages and review logic.
