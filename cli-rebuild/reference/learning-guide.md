# Learning guide

If you are opening `cli-rebuild/` for the first time, the easiest way to understand it is to move through it in this order.

## 1. Look at the building blocks

Run:

```bash
npm run inspect
```

This shows the loaded prompt layers, reminders, tools, agents, skills, workflows, and scenarios. The important idea is that the runtime is not hardcoded into one huge file. Most of the behavior is assembled from modules under [`../modules/`](../modules/).

## 2. Watch a workflow fire

Run:

```bash
npm run trace -- --task "review the current diff for bugs and missing tests" --scenario review
```

That output shows:

- which prompt layers are active
- which reminder text was injected
- which leaked skill-like capabilities are relevant
- which workflow was matched
- how multiple agents are sequenced

## 3. Change one piece

Try editing one file, such as:

- [`../modules/workflows/review.mjs`](../modules/workflows/review.mjs)
- [`../modules/agents/review.mjs`](../modules/agents/review.mjs)
- [`../modules/reminders/review-mode.mjs`](../modules/reminders/review-mode.mjs)

Then run the trace again.

This is the core learning loop of the simulator: small module change, rerun, observe how behavior shifts.

## 4. Use the TUI

Run:

```bash
npm run tui
```

The TUI is intentionally simple. Its job is to make exploration fast, not to hide the structure.

## 5. Compare with the docs

Then read:

- [`../../en/what-leaked.md`](../../en/what-leaked.md)
- [`../../en/how-claude-code-seems-structured.md`](../../en/how-claude-code-seems-structured.md)
- [`leaked-tools-and-skills.md`](leaked-tools-and-skills.md)

That closes the loop between public evidence and the simulator.
