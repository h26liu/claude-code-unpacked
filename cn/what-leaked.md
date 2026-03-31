# 到底泄露了什么

很多人说“Claude Code 的 npm 发布把 system prompt 泄露了”，但更准确的说法是：公开发布的客户端包里包含了足够多的 prompt 和工具定义文本，使外界能够反推出它相当大一部分行为架构。

泄露的不是一个完整的内部源码仓库，而是一个可执行的发布产物。真正有价值的，是里面嵌入的大量 prompt 文本：

- 主 system prompt 片段
- 内置工具描述
- subagent prompts
- 用于总结、压缩、记忆、评审等流程的 utility prompts
- 运行时 reminder 文本
