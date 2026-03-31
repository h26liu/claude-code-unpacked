# Prompt layers

The extracted prompt catalogs are valuable because they show that Claude Code appears to have many prompt layers rather than one.

Those layers include:

- main system prompt text
- builtin tool descriptions
- agent prompts
- utility prompts
- system reminders

That layering is the central architectural lesson of the leak.

Piebald's maintained catalog describes Claude Code as shipping more than a hundred extracted strings, spread across:

- main system prompt components
- conditional prompt fragments
- built-in tool descriptions
- subagent prompts
- utility prompts
- system reminders
- embedded data and templates

In other words, Claude Code appears to be composed from many small prompt modules that can be conditionally activated rather than one static master prompt.
