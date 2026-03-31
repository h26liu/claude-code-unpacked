# Claude Code 看起来是如何组织起来的

这份文档结合了两类证据：

- Anthropic 官方文档
- 基于公开提取 prompt 清单的合理推断

Claude Code 看起来并不是由一个单体 system prompt 驱动，而是由许多层拼起来的：

- 主 system prompt
- 工具描述层
- agent / subagent 层
- memory 层
- runtime reminders 层
