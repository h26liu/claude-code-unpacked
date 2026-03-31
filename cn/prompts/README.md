# Prompt 分层

提取出来的 prompt 清单显示，Claude Code 看起来并不是只有一个 prompt，而是有很多层：

- 主 system prompt
- 内置工具描述
- agent prompts
- utility prompts
- system reminders

这其实是整次泄露最有价值的架构信息。

Piebald 的持续提取清单把这些内容拆成了很多类别，说明 Claude Code 更像是由许多小 prompt 模块按条件拼起来，而不是由一个静态的巨大 master prompt 驱动。
