# Skills

This is the least certain part of the reconstruction.

Anthropic's public docs emphasize memories, subagents, hooks, commands, and MCP. They do not foreground `skills` nearly as clearly.

The leaked prompt catalogs, however, do give us some concrete evidence that a skill-like abstraction exists inside Claude Code's runtime. The clearest references are:

- the `Skill` tool description
- `Remember skill`
- `Skillify Current Session`

There is also a `Claude guide agent`, which may not itself be a skill, but looks like a reusable guidance capability rather than a one-off conversational instruction.

## What can be said with confidence

- A `Skill` tool exists in the extracted tool descriptions.
- Skill-related prompt names exist in the extracted utility and system prompt catalog.
- This means "skills" are not just a fan theory layered on top of the leak.

## What is still inference

- It is not fully clear how exposed `skills` are to end users compared with subagents or slash commands.
- It is not fully clear whether a skill is best understood as a reusable workflow, a prompt bundle, a capability pack, or some hybrid of those.
- The runtime boundary between `skill`, `agent`, `utility`, and `workflow` is still blurrier than the boundary around documented features like hooks or memory.

So the careful position is: skills appear to be real, but they are inferred from extracted runtime text more than they are documented as a stable public product surface.
