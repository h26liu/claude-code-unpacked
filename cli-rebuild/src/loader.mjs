import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";

const categories = ["layers", "reminders", "tools", "agents", "skills", "workflows", "scenarios"];

async function loadDirectory(rootDir, category) {
  const directory = path.join(rootDir, "..", "modules", category);
  const entries = await fs.readdir(directory, { withFileTypes: true });
  const files = entries
    .filter((entry) => entry.isFile() && entry.name.endsWith(".mjs"))
    .map((entry) => entry.name)
    .sort();

  const items = [];

  for (const file of files) {
    const imported = await import(pathToFileURL(path.join(directory, file)).href);
    items.push(imported.default);
  }

  return items;
}

export async function loadCatalog() {
  const rootDir = path.dirname(fileURLToPath(import.meta.url));
  const catalog = {};

  for (const category of categories) {
    catalog[category] = await loadDirectory(rootDir, category);
  }

  return catalog;
}
