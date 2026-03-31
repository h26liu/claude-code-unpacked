# Example scenarios

## Inspect the runtime pieces

```bash
npm run inspect
```

## Start the interactive playground

```bash
npm run tui
```

## Inspect the leaked/public inventory summary

Read:

- [`../reference/leaked-tools-and-skills.md`](../reference/leaked-tools-and-skills.md)
- [`../reference/learning-guide.md`](../reference/learning-guide.md)

## See how plan mode changes routing

```bash
npm run trace -- --task "design a new agent system" --scenario planning --plan-mode
```

## Simulate a code review flow

```bash
npm run trace -- --task "review the current diff for bugs and missing tests" --scenario review
```

## Add extra memory

```bash
npm run trace -- --task "add hooks docs" --memory "Prefer docs with practical examples"
```
