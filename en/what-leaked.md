# What actually leaked

When people say "the Claude Code npm release leaked its system prompt," they usually compress three different things into one sentence.

## 1. It was a package release, not a source-code repo dump

Anthropic publishes Claude Code as the npm package `@anthropic-ai/claude-code`. That means users download a runnable CLI bundle to their own machines. Once that happens, the shipped files are inspectable.

What became public was not a tidy internal repository. It was a client artifact.

## 2. The interesting part was embedded prompt text

The extracted material people care about is not mostly business logic in the usual sense. It is the natural-language scaffolding that appears to steer the product:

- main system prompt pieces
- builtin tool descriptions
- subagent prompts
- utility prompts for summarization, compaction, memory, and reviews
- mode- and event-specific reminder strings

## 3. The "leak" was really prompt extraction

The best-known public catalog of this material is Piebald's extracted prompt repository. Their description is careful: the repository contains strings extracted from Claude Code's compiled source in the npm release.

Claude Code's public npm package exposed enough embedded prompt and tool-definition text for third parties to reconstruct a large portion of its behavioral architecture.
