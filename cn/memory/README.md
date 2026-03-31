# Memory

Memory 是 Claude Code 里最清晰的一层，因为 Anthropic 官方直接文档化了它。

公开的层级包括：

- 企业级 memory
- 项目级 `./CLAUDE.md`
- 用户级 `~/.claude/CLAUDE.md`
- 本地项目级 `./CLAUDE.local.md`

Anthropic 文档里还明确提到了一些关键行为：

- 用 `# ...` 可以快速写入 memory
- 用 `/memory` 可以直接编辑 memory 文件
- 用 `/init` 可以为仓库生成 `CLAUDE.md`
- 用 `@path` 可以导入其他 memory 文件

而提取出来的 prompt 清单又补上了内部维护层，例如 session memory template、session memory update instructions，以及 `Remember skill`。
