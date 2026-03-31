export default {
  id: "skillify-current-session",
  title: "Skillify current session",
  summary: "Converts a useful interaction pattern into a reusable capability or workflow.",
  source: "Piebald extracted prompt catalog",
  when: ({ task }) => /skill|workflow|reusable|template|playbook/i.test(task)
};
