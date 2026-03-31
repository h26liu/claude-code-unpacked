import { buildTrace, inspectCatalog, renderTrace } from "./runtime.mjs";
import { runTui } from "./tui.mjs";

function parseArgs(argv) {
  const args = { _: [] };

  for (let i = 0; i < argv.length; i += 1) {
    const token = argv[i];

    if (token === "--task") {
      args.task = argv[i + 1];
      i += 1;
      continue;
    }

    if (token === "--memory") {
      args.memory = args.memory || [];
      args.memory.push(argv[i + 1]);
      i += 1;
      continue;
    }

    if (token === "--scenario") {
      args.scenario = argv[i + 1];
      i += 1;
      continue;
    }

    if (token === "--plan-mode") {
      args.planMode = true;
      continue;
    }

    args._.push(token);
  }

  return args;
}

function printUsage() {
  console.log(`Usage:
  node src/index.mjs inspect
  node src/index.mjs trace --task "add hooks docs" [--scenario planning] [--plan-mode] [--memory "Prefer examples"]
  node src/index.mjs tui`);
}

const [command, ...rest] = process.argv.slice(2);
const args = parseArgs(rest);

if (!command) {
  printUsage();
  process.exit(1);
}

if (command === "inspect") {
  console.log(await inspectCatalog());
  process.exit(0);
}

if (command === "trace") {
  if (!args.task) {
    console.error("Missing required flag: --task");
    printUsage();
    process.exit(1);
  }

  const trace = await buildTrace({
    task: args.task,
    planMode: Boolean(args.planMode),
    memory: args.memory || [],
    scenarioId: args.scenario || "default"
  });

  console.log(renderTrace(trace));
  process.exit(0);
}

if (command === "tui") {
  await runTui();
  process.exit(0);
}

console.error(`Unknown command: ${command}`);
printUsage();
process.exit(1);
