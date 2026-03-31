export default {
  id: "planning-delegation",
  title: "Planning-first flow",
  priority: 40,
  reason: "Plan mode should front-load decomposition and delegation before execution.",
  when: ({ planMode }) => planMode,
  stages: [
    {
      title: "Context scan",
      agent: "explore",
      note: "Inspect the workspace and collect just enough context to plan."
    },
    {
      title: "Decomposition",
      agent: "plan",
      note: "Break the work into stages, risks, and handoff points."
    },
    {
      title: "Execution handoff",
      agent: "task",
      note: "Turn the plan into a bounded implementation packet."
    }
  ]
};
