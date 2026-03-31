import readline from "node:readline/promises";
import { stdin as input, stdout as output } from "node:process";
import { color } from "./colors.mjs";
import { buildTrace, inspectCatalog, renderTrace } from "./runtime.mjs";

function divider() {
  return `${color.dim("-".repeat(72))}\n`;
}

export async function runTui() {
  const rl = readline.createInterface({ input, output });

  try {
    output.write(`${color.bold("Claude Code clean-room playground")}\n`);
    output.write(`${color.dim("Modular multi-agent simulator with file-based building blocks.")}\n`);
    output.write(divider());

    while (true) {
      output.write(`${color.cyan("Choose an action")}\n`);
      output.write("1. Inspect runtime catalog\n");
      output.write("2. Run a custom trace\n");
      output.write("3. Run a review scenario\n");
      output.write("4. Run a planning scenario\n");
      output.write("5. Exit\n");

      const choice = (await rl.question("> ")).trim();

      if (choice === "1") {
        output.write(divider());
        output.write(`${await inspectCatalog()}\n`);
        output.write(divider());
        continue;
      }

      if (choice === "2") {
        const task = (await rl.question("Task: ")).trim();
        const scenarioId = (await rl.question("Scenario id [default]: ")).trim() || "default";
        const planMode = /^y(es)?$/i.test((await rl.question("Plan mode? [y/N]: ")).trim());
        const memoryInput = (await rl.question("Extra memory, comma separated: ")).trim();
        const memory = memoryInput
          ? memoryInput
              .split(",")
              .map((item) => item.trim())
              .filter(Boolean)
          : [];

        const trace = await buildTrace({ task, scenarioId, planMode, memory });
        output.write(divider());
        output.write(`${renderTrace(trace)}\n`);
        output.write(divider());
        continue;
      }

      if (choice === "3") {
        const trace = await buildTrace({
          task: "review the current diff for bugs, regressions, and missing tests",
          scenarioId: "review"
        });
        output.write(divider());
        output.write(`${renderTrace(trace)}\n`);
        output.write(divider());
        continue;
      }

      if (choice === "4") {
        const trace = await buildTrace({
          task: "design a modular agent system for a coding CLI",
          scenarioId: "planning",
          planMode: true
        });
        output.write(divider());
        output.write(`${renderTrace(trace)}\n`);
        output.write(divider());
        continue;
      }

      if (choice === "5" || choice.toLowerCase() === "exit") {
        break;
      }

      output.write(`${color.yellow("Unknown choice.")}\n`);
    }
  } finally {
    rl.close();
  }
}
