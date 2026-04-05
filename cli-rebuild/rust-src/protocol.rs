use anyhow::{Result, bail};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::tools::ToolSpec;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolEnvelope {
    #[serde(rename = "type")]
    pub envelope_type: String,
    #[serde(default)]
    pub tool: String,
    #[serde(default)]
    pub input: Value,
    #[serde(default)]
    pub message: String,
    #[serde(default)]
    pub summary: String,
}

fn strip_code_fence(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.starts_with("```") && trimmed.ends_with("```") {
        let lines = trimmed.lines().collect::<Vec<_>>();
        if lines.len() >= 3 {
            return lines[1..lines.len() - 1].join("
").trim().to_string();
        }
    }
    trimmed.to_string()
}

pub fn parse_model_envelope(text: &str) -> Result<ToolEnvelope> {
    let candidate = strip_code_fence(text);
    let envelope: ToolEnvelope = serde_json::from_str(&candidate)
        .map_err(|_| anyhow::anyhow!("failed to parse model response as JSON envelope\n{}", text))?;

    if envelope.envelope_type.is_empty() {
        bail!("model response missing type field");
    }

    Ok(envelope)
}

pub fn build_protocol_instructions(tools: &[ToolSpec]) -> String {
    let tool_lines = tools
        .iter()
        .map(|tool| format!("- {} [{}]: {}. Input schema: {}", tool.id, if tool.executable { "executable" } else { "catalog-only" }, tool.use_text, tool.input_schema))
        .collect::<Vec<_>>()
        .join("
");

    [
        "You are operating inside an agentic coding CLI.",
        "You must respond with JSON only. Do not wrap it in markdown.",
        "Allowed response shapes:",
        r#"{"type":"tool_call","tool":"ReadFile","input":{"path":"README.md"},"summary":"Why this tool is needed"}"#,
        r#"{"type":"final","message":"Your final answer to the user","summary":"Short execution summary"}"#,
        r#"{"type":"assistant","message":"A short progress update or question","summary":"Why you need this"}"#,
        "Never invent tools. Use only the executable tools below for actual calls.",
        "Catalog-only tools describe the broader reconstructed Claude Code surface but cannot be executed directly.",
        "Prefer tool calls when the task requires filesystem inspection, code edits, command execution, web access, or delegation.",
        "If you have enough information and the work is complete, return a final response.",
        "If you need to delegate bounded work, use Task with an agent and task field.",
        "",
        "Tool surface:",
        &tool_lines,
    ]
    .join("
")
}
