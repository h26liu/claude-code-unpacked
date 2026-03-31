export default {
  id: "docs-mode",
  title: "Documentation reminder",
  when: ({ task }) => /docs?|readme|explain|writeup|blog/i.test(task),
  text: "This looks documentation-heavy. Optimize for clarity, structure, and examples."
};
