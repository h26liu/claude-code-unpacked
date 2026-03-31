export default {
  id: "review-swarm",
  title: "Review swarm flow",
  priority: 80,
  reason: "Review work benefits from context gathering followed by a dedicated review pass.",
  when: ({ task, scenarioId }) => scenarioId === "review" || /review|diff|regression|bug|tests?/i.test(task),
  stages: [
    {
      title: "Diff scan",
      agent: "explore",
      note: "Locate the changed files and summarize the behavioral surface."
    },
    {
      title: "Risk review",
      agent: "review",
      note: "Inspect correctness issues, regressions, and missing tests."
    }
  ]
};
