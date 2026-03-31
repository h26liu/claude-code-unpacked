# Memory

Memory is one of the clearest parts of Claude Code because Anthropic documents it directly.

The documented hierarchy includes:

- enterprise memory
- project memory via `./CLAUDE.md`
- user memory via `~/.claude/CLAUDE.md`
- local project memory via `./CLAUDE.local.md`

Anthropic also documents a few important behaviors around that hierarchy:

- `# ...` can quickly turn a user message into a saved memory
- `/memory` opens and edits memory files
- `/init` can bootstrap a `CLAUDE.md` for a repo
- `@path` imports let one memory file pull in others

The extracted prompt inventory adds a second layer on top of that documented system:

- session memory update instructions
- a session memory template
- prompts for updating long-term learnings
- a `Remember skill` tied to recurring patterns and memory updates

So memory in Claude Code appears to be both a user-visible feature and an internal maintenance workflow.
