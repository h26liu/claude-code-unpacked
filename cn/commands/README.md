# Commands

Anthropic 官方文档里有两层命令系统：

- 内置 slash commands
- 放在 `.claude/commands/` 里的自定义 markdown commands

而从提取出来的 prompt 看，某些命令似乎还有自己的专用 prompt，例如：

- `/pr-comments`
- `/review-pr`
- `/security-review`

这说明 commands 很可能不只是命令入口，而是绑定了特定 workflow 的 prompt 包。
