import { formatList, formatSection } from "./format.mjs";
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

function selectTools(catalog, agent) {
  return catalog.tools.filter((tool) => agent.tools.includes(tool.id));
}

function selectSkills(catalog, state) {
  return catalog.skills.filter((skill) => !skill.when || skill.when(state));
}

function activeLayers(catalog, state) {
  return catalog.layers.filter((layer) => !layer.when || layer.when(state));
}

function activeReminders(catalog, state) {
  return catalog.reminders.filter((reminder) => reminder.when(state));
}

function renderStageNote(stage, state) {
  return typeof stage.note === "function" ? stage.note(state) : stage.note;
}

export async function inspectCatalog() {
  const catalog = await loadCatalog();

  return [
    "# Clean-room runtime catalog",
    "",
    formatSection("Prompt layers", formatList(catalog.layers.map((item) => `${item.id}: ${item.title}`))),
    formatSection("Reminders", formatList(catalog.reminders.map((item) => `${item.id}: ${item.title}`))),
    formatSection("Tools", formatList(catalog.tools.map((item) => `${item.id}: ${item.use}`))),
    formatSection("Agents", formatList(catalog.agents.map((item) => `${item.id}: ${item.description}`))),
    formatSection("Skills", formatList(catalog.skills.map((item) => `${item.id}: ${item.summary}`))),
    formatSection("Workflows", formatList(catalog.workflows.map((item) => `${item.id}: ${item.title}`))),
    formatSection("Scenarios", formatList(catalog.scenarios.map((item) => `${item.id}: ${item.title}`)))
  ].join("\n");
}

export async function buildTrace({ task, planMode = false, memory = [], scenarioId = "default" }) {
  const catalog = await loadCatalog();
  const scenario = findScenario(catalog, scenarioId);
  const state = {
    task,
    planMode,
    scenarioId: scenario.id,
    memory: [...(scenario.defaultMemory || []), ...memory]
  };

  const layers = activeLayers(catalog, state);
  const reminders = activeReminders(catalog, state);
  const skills = selectSkills(catalog, state);
  const workflow = selectWorkflow(catalog, state);
  const agentSteps = workflow.stages.map((stage, index) => {
    const agent = findAgent(catalog, stage.agent);
    const tools = selectTools(catalog, agent);

    return {
      index: index + 1,
      title: stage.title,
      note: renderStageNote(stage, state),
      agent,
      tools
    };
  });

  return {
    scenario,
    workflow,
    layers,
    reminders,
    skills,
    memory: state.memory,
    operatingBrief: [
      ...layers.map((layer) => layer.text),
      ...state.memory.map((entry) => `Memory: ${entry}`),
      ...reminders.map((entry) => `Reminder: ${entry.text}`),
      ...skills.map((entry) => `Skill available: ${entry.title}`),
      `Workflow: ${workflow.title}`
    ],
    decisions: [
      `Task received: ${task}`,
      `Scenario: ${scenario.title}`,
      `Plan mode: ${planMode ? "on" : "off"}`,
      `Matched workflow: ${workflow.id}`,
      `Why: ${workflow.reason}`
    ],
    agentSteps
  };
}

export function renderTrace(trace) {
  return [
    "# Session trace",
    "",
    formatSection("Selected prompt layers", formatList(trace.layers.map((layer) => `${layer.title} (${layer.id})`))),
    formatSection("Active memory", formatList(trace.memory)),
    formatSection(
      "Active reminders",
      trace.reminders.length ? formatList(trace.reminders.map((entry) => `${entry.id}: ${entry.text}`)) : "- none"
    ),
    formatSection(
      "Relevant leaked skills",
      trace.skills.length ? formatList(trace.skills.map((entry) => `${entry.title}: ${entry.summary}`)) : "- none"
    ),
    formatSection("Matched workflow", `- ${trace.workflow.title}: ${trace.workflow.reason}`),
    formatSection(
      "Multi-agent flow",
      formatList(
        trace.agentSteps.map(
          (step) =>
            `${step.index}. ${step.title} -> ${step.agent.title} | tools: ${step.tools
              .map((tool) => tool.id)
              .join(", ")} | note: ${step.note}`
        )
      )
    ),
    formatSection("Operating brief", formatList(trace.operatingBrief)),
    formatSection("Decision trace", formatList(trace.decisions))
  ].join("\n");
}
