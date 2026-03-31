# Builtin agents

Based on extracted prompt names, these appear to be the most structurally important builtins:

- `Explore`
- `Plan`
- `Task`

Those three look like the core execution triangle:

- `Explore` for context gathering and repository inspection
- `Plan` for decomposition and sequencing
- `Task` for bounded execution or delegated work

But the extracted prompt inventory goes further. It also points to narrower internal helpers and utilities, including prompts for:

- security review
- CLAUDE.md generation
- agent creation
- statusline setup
- conversation summarization
- recent message summarization
- session title and branch generation
- web fetch summarization

So when people say Claude Code "has agents," the richer reading is that agent-like orchestration appears to exist at multiple levels:

- end-user subagents exposed as a feature
- internal helper agents used by the product runtime itself
