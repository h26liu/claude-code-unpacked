export default {
  id: "claude-guide-agent",
  title: "Claude guide agent",
  summary: "Helps users understand how to use Claude Code, the agent SDK, and related workflows.",
  source: "Piebald extracted prompt catalog",
  when: ({ task }) => /learn|understand|guide|explain/i.test(task)
};
