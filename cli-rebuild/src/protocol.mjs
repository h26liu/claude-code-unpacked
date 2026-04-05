function stripCodeFence(text) {
  const fenced = text.match(/```(?:json)?\s*([\s\S]*?)```/i);
  return fenced ? fenced[1].trim() : text.trim();
}

export function parseModelEnvelope(text) {
  const candidate = stripCodeFence(text);

  try {
    const parsed = JSON.parse(candidate);

    if (!parsed || typeof parsed !== "object") {
      throw new Error("Model response was not a JSON object.");
    }

    if (!parsed.type) {
      throw new Error("Model response missing `type`.");
    }

    return parsed;
  } catch {
    throw new Error(`Failed to parse model response as JSON envelope.\nRaw response:\n${text}`);
  }
}

export function buildProtocolInstructions({ tools }) {
  const toolLines = tools
    .map((tool) => `- ${tool.id}: ${tool.use}. Input schema: ${JSON.stringify(tool.inputSchema || {})}`)
    .join("\n");

  return [
    "You are operating inside an agentic coding CLI.",
    "You must respond with JSON only. Do not wrap it in markdown.",
    "Allowed response shapes:",
    '{"type":"tool_call","tool":"ReadFile","input":{"path":"README.md"},"summary":"Why this tool is needed"}',
    '{"type":"final","message":"Your final answer to the user","summary":"Short execution summary"}',
    '{"type":"assistant","message":"A short progress update or question","summary":"Why you need this"}',
    "Never invent tools. Use only the tool list below.",
    "Prefer tool calls when the task requires filesystem inspection, code edits, command execution, or delegation.",
    "If you have enough information and the work is complete, return `final`.",
    "",
    "Executable tools:",
    toolLines
  ].join("\n");
}
