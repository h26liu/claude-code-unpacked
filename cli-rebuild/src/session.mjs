import { loadCatalog } from "./loader.mjs";

function findScenario(catalog, id) {
  return catalog.scenarios.find((scenario) => scenario.id === id) || catalog.scenarios[0];
}

function findAgent(catalog, id) {
  return catalog.agents.find((agent) => agent.id === id);
}

function selectWorkflow(catalog, state) {
  const matches = catalog.workflows.filter((workflow) => workflow.when(state));

  if (matches.length) {
    return matches.sort((left, right) => (right.priority || 0) - (left.priority || 0))[0];
  }

  return catalog.workflows.find((workflow) => workflow.id === "implementation-default") || catalog.workflows[0];
}

export async function prepareSession({ task, planMode = false, memory = [], scenarioId = "default" }) {
  const catalog = await loadCatalog();
  const scenario = findScenario(catalog, scenarioId);
  const state = {
    task,
    planMode,
    scenarioId: scenario.id,
    memory: [...(scenario.defaultMemory || []), ...memory]
  };

  const layers = catalog.layers.filter((layer) => !layer.when || layer.when(state));
  const reminders = catalog.reminders.filter((reminder) => reminder.when(state));
  const skills = catalog.skills.filter((skill) => !skill.when || skill.when(state));
  const workflow = selectWorkflow(catalog, state);
  const agents = workflow.stages.map((stage) => ({
    ...stage,
    note: typeof stage.note === "function" ? stage.note(state) : stage.note,
    agent: findAgent(catalog, stage.agent)
  }));

  return {
    catalog,
    scenario,
    workflow,
    layers,
    reminders,
    skills,
    agents,
    memory: state.memory
  };
}
