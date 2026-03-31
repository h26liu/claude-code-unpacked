# Hooks

Hooks 是官方文档已经明确存在的一层，它让 Claude Code 更像一个自动化运行时。

常见 hook 事件包括：

- `PreToolUse`
- `PostToolUse`
- `Notification`
- `UserPromptSubmit`
- `Stop`
- `SubagentStop`

这个事件集合说明 Claude Code 内部明显在跟踪多种生命周期：

- 工具调用生命周期
- 用户输入生命周期
- subagent 生命周期
- 会话生命周期

提取出来的 prompt 清单里还出现了 `Agent Hook` 和 `Hook condition evaluator` 这类 utility prompt，进一步说明 hooks 不是边角功能，而是运行时架构的一部分。
