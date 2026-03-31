export default {
  id: "remember-skill",
  title: "Remember skill",
  summary: "Reviews recurring patterns from the session and proposes durable memory updates.",
  source: "Piebald extracted prompt catalog",
  when: ({ task }) => /remember|memory|pattern|learn/i.test(task)
};
