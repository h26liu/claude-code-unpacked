export default {
  id: "implementation-default",
  title: "Default implementation flow",
  priority: 10,
  reason: "Implementation work usually benefits from quick exploration before execution.",
  when: ({ task, planMode }) => !planMode && !/review|diff|regression|bug|tests?/i.test(task),
  stages: [
    {
      title: "Context scan",
      agent: "explore",
      note: "Map the workspace, find relevant files, and identify constraints."
    },
    {
      title: "Execution",
      agent: "task",
      note: "Apply a bounded edit once the target path is clear."
    }
  ]
};
