# Agents

Claude Code 看起来至少以两种方式使用 agent：

- 面向用户的 subagents
- 系统内部用于 planning、exploration、task execution、summarization、memory updates 等工作的 helper agents

从持续提取出来的 prompt 清单看，内部 helper agents 的范围可能比最初想象得更大，还包括：

- security review
- CLAUDE.md 生成
- statusline setup
- 会话总结
- recent message 总结
- session title / branch 生成
- agent creation

所以 Claude Code 更像一个会协调多个专业 worker 的 supervisor，而不是单一助手。
