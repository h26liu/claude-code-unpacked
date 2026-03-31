# Tools

The extracted prompt inventories show that Claude Code's tool surface is much wider than the shorthand description people usually give it.

Anthropic's public docs establish the general picture: Claude Code is a tool-using coding assistant with planning, editing, search, memory, and automation features. The maintained Piebald extraction catalog adds the more granular view by listing the built-in tool descriptions found in the npm bundle.

At a high level, the leaked and public tool layer breaks into these groups:

- file and search tools
- shell and execution tools
- web tools
- planning and task tools
- collaboration tools
- extension and discovery tools

That matters because tool descriptions are not just implementation details. In products like Claude Code, they are part of the behavioral interface presented to the model.

## What the extracted catalog strongly supports

Piebald's README describes a maintained list of Claude Code tool descriptions extracted from the compiled npm package. As of the catalog snapshot for Claude Code `v2.1.77` on March 16, 2026, the tool layer includes:

- local file tools such as `ReadFile`, `Write`, `Edit`, `Glob`, and `Grep`
- shell and runtime tools such as `Bash`, `Sleep`, and `NotebookEdit`
- web tools such as `WebSearch` and `WebFetch`
- planning tools such as `EnterPlanMode`, `ExitPlanMode`, and `TodoWrite`
- delegation tools such as `Task` and `TaskCreate`
- coordination tools such as `SendMessageTool`, `TeammateTool`, and `TeamDelete`
- capability and discovery tools such as `Skill`, `ToolSearch`, and `LSP`
- interactive or automation tools such as `AskUserQuestion` and `Computer`

That is one of the clearest signals from the leak: Claude Code appears to be a fairly large runtime with explicit support for planning, delegation, and coordination, not just a single assistant calling shell commands.
