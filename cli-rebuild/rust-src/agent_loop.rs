use anyhow::{Result, bail};

use crate::{
    config::Config,
    protocol::{ToolEnvelope, build_protocol_instructions, parse_model_envelope},
    providers::{Message, call_model},
    session::{PreparedSession, prepare_session},
    tools::{ToolContext, executable_tools, run_tool, tool_catalog},
};

#[derive(Debug, Clone)]
pub struct RunResult {
    pub session: PreparedSession,
    pub final_message: String,
    pub transcript: Vec<ToolEnvelope>,
}

fn build_system_prompt(session: &PreparedSession, config: &Config) -> String {
    let tools = executable_tools();
    let mut parts = Vec::new();

    parts.extend(session.layers.iter().map(|layer| layer.text.to_string()));
    parts.push(format!("Scenario: {}", session.scenario.title));
    parts.extend(session.memory.iter().map(|entry| format!("Memory: {}", entry)));
    parts.extend(session.memory_sources.iter().map(|source| {
        format!("{} from {}:\n{}", source.label, source.path.display(), source.content.trim())
    }));
    parts.extend(session.reminders.iter().map(|entry| format!("Reminder: {}", entry.text)));
    parts.extend(session.skills.iter().map(|entry| format!("Skill available: {} - {}", entry.title, entry.summary)));

    if !session.command_context.is_empty() {
        parts.push(format!("Local command context:\n{}", session.command_context.join("\n\n")));
    }

    if !session.loaded_commands.is_empty() {
        parts.push(format!(
            "Loaded local commands:\n{}",
            session.loaded_commands.iter().map(|command| format!("- /{}: {}", command.id, command.summary)).collect::<Vec<_>>().join("\n")
        ));
    }

    if !session.loaded_agents.is_empty() {
        parts.push(format!(
            "Loaded local agents:\n{}",
            session.loaded_agents.iter().map(|agent| format!("- {}: {} | tools: {}\n{}", agent.id, agent.summary, if agent.tools.is_empty() { "none".to_string() } else { agent.tools.join(", ") }, agent.body)).collect::<Vec<_>>().join("\n\n")
        ));
    }

    if !session.loaded_prompts.is_empty() {
        parts.push(format!(
            "Loaded local prompt overlays:\n{}",
            session.loaded_prompts.iter().map(|prompt| format!("- {} ({})\n{}", prompt.title, prompt.path.display(), prompt.body)).collect::<Vec<_>>().join("\n\n")
        ));
    }

    parts.push(format!("Workflow: {}", session.workflow.title));
    parts.push(format!("Workflow reason: {}", session.workflow.reason));
    parts.extend(session.agents.iter().enumerate().map(|(index, (title, agent, note))| {
        format!("Stage {}: {} -> {}. {}", index + 1, title, agent.title, note)
    }));

    if !config.hooks.pre_tool.is_empty() || !config.hooks.post_tool.is_empty() || !config.hooks.on_final.is_empty() {
        parts.push(format!(
            "Configured hooks: pre_tool=[{}], post_tool=[{}], on_final=[{}]",
            config.hooks.pre_tool.join(", "),
            config.hooks.post_tool.join(", "),
            config.hooks.on_final.join(", ")
        ));
    }

    parts.push(build_protocol_instructions(&tools));
    parts.join("\n\n")
}

fn render_tool_result(tool_name: &str, result: &str) -> String {
    format!("Tool result: {}\n{}", tool_name, result)
}

fn apply_hooks(stage: &str, hooks: &[String], target: &str) -> Option<String> {
    if hooks.is_empty() {
        None
    } else {
        Some(format!("Hook stage={} target={} hooks={}", stage, target, hooks.join(", ")))
    }
}

async fn run_subagent(
    provider: &crate::config::ProviderConfig,
    runtime: &crate::config::RuntimeConfig,
    session: &PreparedSession,
    agent_id: &str,
    task: &str,
    interactive: bool,
) -> Result<String> {
    let agent = session
        .catalog
        .agents
        .iter()
        .find(|candidate| candidate.id == agent_id)
        .cloned()
        .ok_or_else(|| anyhow::anyhow!("unknown subagent: {}", agent_id))?;
    let tools = executable_tools()
        .into_iter()
        .filter(|tool| agent.tools.iter().any(|item| item == &tool.id))
        .collect::<Vec<_>>();
    let mut messages = vec![
        Message {
            role: "system".to_string(),
            content: [
                format!("{}: {}", agent.title, agent.description),
                session.memory.iter().map(|entry| format!("Memory: {}", entry)).collect::<Vec<_>>().join("\n"),
                build_protocol_instructions(&tools),
            ]
            .join("\n\n"),
        },
        Message {
            role: "user".to_string(),
            content: format!("Subtask: {}", task),
        },
    ];
    let mut context = ToolContext {
        interactive,
        ..Default::default()
    };

    for _ in 0..4 {
        let raw = call_model(provider, &messages).await?;
        let envelope = parse_model_envelope(&raw)?;
        match envelope.envelope_type.as_str() {
            "final" | "assistant" => {
                return Ok(if !envelope.message.is_empty() {
                    envelope.message
                } else {
                    envelope.summary
                })
            }
            "tool_call" => {
                if envelope.tool == "Task" {
                    return Ok("Nested Task delegation is disabled in subagents.".to_string());
                }
                let result = run_tool(&envelope.tool, &envelope.input, runtime, &mut context).await?;
                messages.push(Message { role: "assistant".to_string(), content: raw });
                messages.push(Message { role: "user".to_string(), content: render_tool_result(&envelope.tool, &result) });
            }
            other => bail!("unsupported subagent envelope type: {}", other),
        }
    }

    Ok("Subagent stopped after reaching the turn limit.".to_string())
}

pub async fn run_task(
    config: &Config,
    task: &str,
    scenario_id: &str,
    plan_mode: bool,
    extra_memory: &[String],
    interactive: bool,
) -> Result<RunResult> {
    let session = prepare_session(task, scenario_id, plan_mode, extra_memory, &config.runtime.workspace_root);
    let mut messages = vec![
        Message { role: "system".to_string(), content: build_system_prompt(&session, config) },
        Message { role: "user".to_string(), content: task.to_string() },
    ];
    let mut context = ToolContext {
        interactive,
        plan_mode,
        ..Default::default()
    };
    let mut transcript = Vec::new();

    for _ in 0..config.runtime.max_turns {
        let raw = call_model(&config.provider, &messages).await?;
        let envelope = parse_model_envelope(&raw)?;
        transcript.push(envelope.clone());

        match envelope.envelope_type.as_str() {
            "final" => {
                let final_message = if let Some(hook_message) = apply_hooks("on-final", &config.hooks.on_final, "final") {
                    format!("{}\n\n{}", hook_message, envelope.message)
                } else {
                    envelope.message
                };
                return Ok(RunResult { session, final_message, transcript });
            }
            "assistant" => {
                messages.push(Message {
                    role: "assistant".to_string(),
                    content: if !envelope.message.is_empty() { envelope.message } else { envelope.summary },
                });
            }
            "tool_call" => {
                if let Some(hook_message) = apply_hooks("pre-tool", &config.hooks.pre_tool, &envelope.tool) {
                    messages.push(Message { role: "system".to_string(), content: hook_message });
                }

                let result = if envelope.tool == "Task" {
                    let agent_id = envelope.input.get("agent").and_then(|value| value.as_str()).unwrap_or("task");
                    let subtask = envelope.input.get("task").and_then(|value| value.as_str()).unwrap_or("");
                    run_subagent(&config.provider, &config.runtime, &session, agent_id, subtask, interactive).await?
                } else {
                    run_tool(&envelope.tool, &envelope.input, &config.runtime, &mut context).await?
                };

                messages.push(Message { role: "assistant".to_string(), content: raw });
                messages.push(Message { role: "user".to_string(), content: render_tool_result(&envelope.tool, &result) });

                if let Some(hook_message) = apply_hooks("post-tool", &config.hooks.post_tool, &envelope.tool) {
                    messages.push(Message { role: "system".to_string(), content: hook_message });
                }
            }
            other => bail!("unsupported envelope type: {}", other),
        }
    }

    bail!("agent loop reached the maximum number of turns without a final answer")
}

pub fn inspect_catalog() -> String {
    let workspace_root = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
    let session = prepare_session("understand the repo", "default", false, &[], &workspace_root);
    let tools = tool_catalog();

    [
        "# Rust runtime catalog".to_string(),
        "".to_string(),
        "## Prompt layers".to_string(),
        session.layers.iter().map(|item| format!("- {}: {}", item.id, item.title)).collect::<Vec<_>>().join("\n"),
        "".to_string(),
        "## Agents".to_string(),
        session.catalog.agents.iter().map(|item| format!("- {}: {}", item.id, item.description)).collect::<Vec<_>>().join("\n"),
        "".to_string(),
        "## Skills".to_string(),
        session.catalog.skills.iter().map(|item| format!("- {}: {}", item.id, item.summary)).collect::<Vec<_>>().join("\n"),
        "".to_string(),
        "## Commands".to_string(),
        session.catalog.commands.iter().map(|item| format!("- {}: {}", item.id, item.summary)).collect::<Vec<_>>().join("\n"),
        if session.loaded_commands.is_empty() { "".to_string() } else { format!("\n## Loaded local commands\n{}", session.loaded_commands.iter().map(|item| format!("- /{}: {} ({})", item.id, item.summary, item.path.display())).collect::<Vec<_>>().join("\n")) },
        "".to_string(),
        "## Hooks".to_string(),
        session.catalog.hooks.iter().map(|item| format!("- {}: {}", item.id, item.summary)).collect::<Vec<_>>().join("\n"),
        if session.loaded_agents.is_empty() { "".to_string() } else { format!("\n## Loaded local agents\n{}", session.loaded_agents.iter().map(|item| format!("- {}: {} | tools: {}", item.id, item.summary, if item.tools.is_empty() { "none".to_string() } else { item.tools.join(", ") })).collect::<Vec<_>>().join("\n")) },
        if session.loaded_prompts.is_empty() { "".to_string() } else { format!("\n## Loaded local prompts\n{}", session.loaded_prompts.iter().map(|item| format!("- {} ({})", item.title, item.path.display())).collect::<Vec<_>>().join("\n")) },
        "".to_string(),
        "## Tool surface".to_string(),
        tools.iter().map(|item| format!("- {} [{}]: {}", item.id, if item.executable { "executable" } else { "catalog-only" }, item.use_text)).collect::<Vec<_>>().join("\n"),
        "".to_string(),
        "## Workflows".to_string(),
        session.catalog.workflows.iter().map(|item| format!("- {}: {}", item.id, item.title)).collect::<Vec<_>>().join("\n"),
    ]
    .join("\n")
}

pub fn render_trace(config: &Config, task: &str, scenario_id: &str, plan_mode: bool, extra_memory: &[String]) -> String {
    let session = prepare_session(task, scenario_id, plan_mode, extra_memory, &config.runtime.workspace_root);
    [
        "# Session trace".to_string(),
        "".to_string(),
        "## Selected prompt layers".to_string(),
        session.layers.iter().map(|layer| format!("- {} ({})", layer.title, layer.id)).collect::<Vec<_>>().join("\n"),
        "".to_string(),
        "## Active memory".to_string(),
        session.memory.iter().map(|entry| format!("- {}", entry)).collect::<Vec<_>>().join("\n"),
        "".to_string(),
        "## Loaded memory files".to_string(),
        if session.memory_sources.is_empty() {
            "- none".to_string()
        } else {
            session.memory_sources.iter().map(|source| format!("- {}: {}", source.label, source.path.display())).collect::<Vec<_>>().join("\n")
        },
        "".to_string(),
        "## Active reminders".to_string(),
        if session.reminders.is_empty() { "- none".to_string() } else { session.reminders.iter().map(|entry| format!("- {}: {}", entry.id, entry.text)).collect::<Vec<_>>().join("\n") },
        "".to_string(),
        "## Relevant leaked skills".to_string(),
        if session.skills.is_empty() { "- none".to_string() } else { session.skills.iter().map(|entry| format!("- {}: {}", entry.title, entry.summary)).collect::<Vec<_>>().join("\n") },
        "".to_string(),
        "## Loaded local Claude files".to_string(),
        format!(
            "- commands: {}\n- agents: {}\n- prompts: {}",
            if session.loaded_commands.is_empty() { "none".to_string() } else { session.loaded_commands.iter().map(|item| format!("/{}", item.id)).collect::<Vec<_>>().join(", ") },
            if session.loaded_agents.is_empty() { "none".to_string() } else { session.loaded_agents.iter().map(|item| item.id.clone()).collect::<Vec<_>>().join(", ") },
            if session.loaded_prompts.is_empty() { "none".to_string() } else { session.loaded_prompts.iter().map(|item| item.id.clone()).collect::<Vec<_>>().join(", ") }
        ),
        "".to_string(),
        "## Applied local command context".to_string(),
        if session.command_context.is_empty() { "- none".to_string() } else { session.command_context.iter().map(|item| format!("- {}", item.replace('\n', " "))).collect::<Vec<_>>().join("\n") },
        "".to_string(),
        "## Matched workflow".to_string(),
        format!("- {}: {}", session.workflow.title, session.workflow.reason),
        "".to_string(),
        "## Multi-agent flow".to_string(),
        session.agents.iter().enumerate().map(|(index, (title, agent, note))| format!("- {}. {} -> {} | tools: {} | note: {}", index + 1, title, agent.title, agent.tools.join(", "), note)).collect::<Vec<_>>().join("\n"),
        "".to_string(),
        "## Runtime config".to_string(),
        format!("- provider: {} / {}\n- workspace: {}\n- max_turns: {}\n- hooks: pre_tool={} post_tool={} on_final={}", config.provider.r#type, config.provider.model, config.runtime.workspace_root.display(), config.runtime.max_turns, config.hooks.pre_tool.join(", "), config.hooks.post_tool.join(", "), config.hooks.on_final.join(", ")),
    ]
    .join("\n")
}
