# Claude Code clean-room CLI rebuild

This directory is an educational simulator, not a clone.

The goal is to let readers play with a Claude Code-like architecture locally and see how a prompt-driven coding CLI can be assembled from:

- system prompt layers
- memory files
- runtime reminders
- tools
- builtin agents
- multi-agent workflows
- pluggable modules that can be swapped like Lego pieces

It does not implement Anthropic's real product logic, and it does not claim to expose private chain-of-thought. What it does expose is a clear execution trace showing which layers were activated, which workflow was selected, which agents participated, which tools were surfaced, and why.

There is also an important boundary here: this project is meant to teach orchestration, not to impersonate Anthropic's internal runtime. Where the public evidence is strong, the simulator names that clearly. Where the repo makes a design choice to keep the simulator useful, it says so.

## Why this exists

The documentation in the root of this repo explains what leaked and how Claude Code appears to be structured. This folder turns that explanation into something runnable.

That makes it easier to answer questions like:

- What changes when plan mode is active?
- Why would a coding CLI delegate to a planning agent instead of a task agent?
- How do memory, reminders, and tool descriptions combine into one effective prompt?
- What does a modular prompt runtime feel like in practice?

## Structure

- [`package.json`](package.json): local scripts
- [`src/index.mjs`](src/index.mjs): CLI entrypoint
- [`src/runtime.mjs`](src/runtime.mjs): prompt assembly and workflow execution
- [`src/tui.mjs`](src/tui.mjs): interactive terminal playground
- [`src/loader.mjs`](src/loader.mjs): file-based module loader
- [`modules/`](modules): prompt layers, reminders, tools, agents, workflows, and scenarios
- [`reference/leaked-tools-and-skills.md`](reference/leaked-tools-and-skills.md): sourced inventory notes from public evidence
- [`reference/learning-guide.md`](reference/learning-guide.md): how to learn the simulator step by step
- [`examples/scenarios.md`](examples/scenarios.md): suggested demo commands

## Quick start

```bash
cd cli-rebuild
npm run inspect -- --task "understand the repo"
npm run trace -- --task "add a hooks guide" --scenario planning --plan-mode
npm run trace -- --task "review the current diff" --scenario review --memory "Prefer small reviewable patches"
npm run tui
```

## Commands

### `inspect`

Shows the reconstructed catalog: prompt layers, agents, tools, reminders, workflows, and scenarios.

### `trace`

Builds a simulated session and prints:

- selected prompt layers
- active memories
- runtime reminders
- matched workflow
- mocked multi-agent delegation flow
- assembled operating brief
- a decision trace

### `tui`

Starts an interactive terminal playground for exploring the runtime without remembering flags.

## Design choices

This simulator intentionally uses the phrase `decision trace` instead of `reasoning dump`.

That is deliberate. The point is to make orchestration legible, not to pretend we have access to hidden internal reasoning. The output should help a reader understand the runtime shape:

- what context is assembled
- what gets prioritized
- what gets delegated
- what tools are surfaced

## How to extend it

The runtime is file-driven by design. To change the behavior, edit or add modules under [`modules/`](modules):

- [`modules/tools/`](modules/tools)
- [`modules/agents/`](modules/agents)
- [`modules/reminders/`](modules/reminders)
- [`modules/workflows/`](modules/workflows)
- [`modules/scenarios/`](modules/scenarios)

That keeps the project easy to remix. A user can try a new routing strategy, add another review agent, or swap memory stacks without rewriting the core runtime.

## What was added from the leaked/public inventory

The simulator is now informed by the maintained public extraction catalog from Piebald, especially for:

- the built-in tool surface
- the presence of `Plan`, `Explore`, and `Task` style agents
- skill-related prompt names such as `Remember skill` and `Skillify Current Session`

See [`reference/leaked-tools-and-skills.md`](reference/leaked-tools-and-skills.md) for the sourced summary and caveats.

## Relationship to the rest of the repo

Use this folder together with:

- [`../en/what-leaked.md`](../en/what-leaked.md)
- [`../en/how-claude-code-seems-structured.md`](../en/how-claude-code-seems-structured.md)
- [`../en/agents/README.md`](../en/agents/README.md)
- [`../en/prompts/README.md`](../en/prompts/README.md)

The docs explain the architecture. This directory lets readers poke at it.
