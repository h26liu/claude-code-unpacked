export default {
  id: "review-mode",
  title: "Review reminder",
  when: ({ task, scenarioId }) => scenarioId === "review" || /review|diff|regression|bug|tests?/i.test(task),
  text: "This looks like a review flow. Prioritize correctness, regressions, and missing tests."
};
