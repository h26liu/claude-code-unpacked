export default {
  id: "swarm-design",
  title: "Swarm design flow",
  priority: 90,
  reason: "Coordination-heavy tasks benefit from an explicit orchestration stage before decomposition.",
  when: ({ task }) => /swarm|team|multi-agent|coordination/i.test(task),
  stages: [
    {
      title: "Coordination setup",
      agent: "swarm-coordinator",
      note: "Decide whether the problem should be split and how agents should communicate."
    },
    {
      title: "Planning",
      agent: "plan",
      note: "Break the coordinated work into bounded chunks."
    },
    {
      title: "Execution handoff",
      agent: "task",
      note: "Turn the design into concrete executable tasks."
    }
  ]
};
