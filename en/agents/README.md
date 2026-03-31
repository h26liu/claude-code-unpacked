# Agents

Claude Code appears to use agents in two different ways.

First, there are user-visible subagents, which Anthropic documents officially. Those are markdown-defined specialists with their own descriptions, custom system prompts, separate context windows, and restricted tool sets.

Second, the extracted prompt catalogs suggest a larger internal layer of helper agents and utilities. Piebald's maintained extraction catalog lists not only the expected `Explore`, `Plan`, and `Task` prompts, but also prompts for:

- conversation summarization
- recent message summarization
- session title and branch generation
- session memory updates
- status line setup
- CLAUDE.md creation
- agent creation
- command-description helpers
- web fetch summarization
- security review flows
- hook evaluation and related utility behavior

That is an important structural clue. Claude Code looks less like one assistant with a few extra commands, and more like a supervisor sitting on top of multiple specialized workers.
