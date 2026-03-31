export function formatList(items) {
  if (!items.length) {
    return "- none";
  }

  return items.map((item) => `- ${item}`).join("\n");
}

export function formatSection(title, body) {
  return [`## ${title}`, body, ""].join("\n");
}
