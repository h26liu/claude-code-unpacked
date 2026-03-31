# Claude Code, after the npm prompt leak

Disclaimer: this is an independent, unofficial reconstruction based on public documentation and publicly extracted package artifacts.

## English

This repo is a clean-room reconstruction of how Claude Code appears to be structured, written from two kinds of public evidence:

- Anthropic's official Claude Code documentation
- prompt and tool-description strings extracted from public npm releases of `@anthropic-ai/claude-code`

The main point is simple: Claude Code did not "open source itself" by accident. What became visible was the packaged behavioral layer. Because so much of an agentic CLI's logic lives in prompts, tool specs, reminders, and orchestration text, extracting the npm bundle reveals a surprisingly detailed map of the product.

This repository separates documented facts from inference. Wherever the writeup talks about how Claude Code "appears" to work internally, that language is intentional.

Start here:

- English docs: [en/what-leaked.md](en/what-leaked.md)
- English structure notes: [en/how-claude-code-seems-structured.md](en/how-claude-code-seems-structured.md)
- English index: [en/README.md](en/README.md)

Primary sources:

- Anthropic Claude Code docs: <https://docs.anthropic.com/en/docs/claude-code>
- Anthropic hooks reference: <https://docs.anthropic.com/en/docs/claude-code/hooks>
- Piebald extracted prompt catalog: <https://github.com/Piebald-AI/claude-code-system-prompts>

## 简体中文

这个仓库是一份对 Claude Code 结构的 clean-room 重建，依据主要来自两类公开信息：

- Anthropic 官方 Claude Code 文档
- 从公开 npm 包 `@anthropic-ai/claude-code` 中提取出的 prompt 与工具描述文本

核心结论很简单：Claude Code 并不是“意外开源”了，而是它被打包发布到客户端里的那一层行为逻辑被人提取了出来。由于一个 agentic CLI 的很多关键逻辑本来就存在于 prompts、工具描述、提醒文本和编排规则里，所以从 npm 包中提取字符串，实际上就能还原出相当多的产品结构。

这个仓库会刻意区分“官方文档已经明确说明的内容”和“根据提取结果做出的合理推断”。凡是写成“看起来”“似乎”“大概率”的地方，都不是在假装那是官方源码事实。

从这里开始：

- 中文文档: [cn/what-leaked.md](cn/what-leaked.md)
- 中文结构说明: [cn/how-claude-code-seems-structured.md](cn/how-claude-code-seems-structured.md)
- 中文索引: [cn/README.md](cn/README.md)

主要参考来源：

- Anthropic Claude Code 文档: <https://docs.anthropic.com/en/docs/claude-code>
- Anthropic hooks 文档: <https://docs.anthropic.com/en/docs/claude-code/hooks>
- Piebald 提取 prompt 清单: <https://github.com/Piebald-AI/claude-code-system-prompts>
