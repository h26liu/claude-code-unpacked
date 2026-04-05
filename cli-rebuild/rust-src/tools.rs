use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
    process::Command,
    thread,
    time::Duration,
};

use anyhow::{Context, Result, bail};
use globset::{Glob, GlobSetBuilder};
use serde_json::{Value, json};
use walkdir::WalkDir;

use crate::config::RuntimeConfig;

#[derive(Debug, Clone)]
pub struct ToolSpec {
    pub id: &'static str,
    pub group: &'static str,
    pub use_text: &'static str,
    pub input_schema: &'static str,
    pub executable: bool,
}

#[derive(Debug, Default, Clone)]
pub struct ToolContext {
    pub todos: Vec<String>,
    pub interactive: bool,
    pub plan_mode: bool,
    pub messages: Vec<String>,
    pub teammates: Vec<String>,
}

fn truncate(text: String, max_chars: usize) -> String {
    if text.len() <= max_chars {
        text
    } else {
        format!("{}\n...[truncated {} chars]", &text[..max_chars], text.len() - max_chars)
    }
}

fn resolve_within_root(root: &Path, target: &str) -> Result<PathBuf> {
    let resolved = root.join(target).canonicalize().unwrap_or_else(|_| root.join(target));
    if !resolved.starts_with(root) {
        bail!("path escapes workspace root: {}", target);
    }
    Ok(resolved)
}

fn get_string(input: &Value, field: &str) -> Result<String> {
    input
        .get(field)
        .and_then(|value| value.as_str())
        .map(|value| value.to_string())
        .ok_or_else(|| anyhow::anyhow!("missing string field `{}`", field))
}

pub fn tool_catalog() -> Vec<ToolSpec> {
    vec![
        ToolSpec { id: "AskUserQuestion", group: "interaction", use_text: "Ask the user a short clarifying question.", input_schema: r#"{"question":"string"}"#, executable: true },
        ToolSpec { id: "Bash", group: "execution", use_text: "Run a shell command inside the workspace.", input_schema: r#"{"command":"string"}"#, executable: true },
        ToolSpec { id: "Computer", group: "ui", use_text: "Drive a local computer or browser-like environment.", input_schema: r#"{"action":"string","target":"string?"}"#, executable: false },
        ToolSpec { id: "Edit", group: "files", use_text: "Replace text inside a file.", input_schema: r#"{"path":"string","old_string":"string","new_string":"string","all":"boolean?"}"#, executable: true },
        ToolSpec { id: "EnterPlanMode", group: "mode", use_text: "Enter planning mode and bias the runtime toward decomposition.", input_schema: r#"{}"#, executable: true },
        ToolSpec { id: "ExitPlanMode", group: "mode", use_text: "Exit planning mode.", input_schema: r#"{}"#, executable: true },
        ToolSpec { id: "Glob", group: "search", use_text: "Find files by wildcard pattern.", input_schema: r#"{"pattern":"string"}"#, executable: true },
        ToolSpec { id: "Grep", group: "search", use_text: "Search for text in workspace files.", input_schema: r#"{"query":"string"}"#, executable: true },
        ToolSpec { id: "LSP", group: "analysis", use_text: "Run language-aware code lookup or symbol search.", input_schema: r#"{"query":"string","path":"string?"}"#, executable: true },
        ToolSpec { id: "NotebookEdit", group: "files", use_text: "Edit notebook-like content or write structured cells.", input_schema: r#"{"path":"string","content":"string"}"#, executable: true },
        ToolSpec { id: "ReadFile", group: "files", use_text: "Read a text file from the workspace.", input_schema: r#"{"path":"string"}"#, executable: true },
        ToolSpec { id: "SendMessageTool", group: "coordination", use_text: "Send a message to another agent or teammate.", input_schema: r#"{"to":"string","message":"string"}"#, executable: true },
        ToolSpec { id: "Skill", group: "skills", use_text: "Inspect or apply a reusable skill-like workflow.", input_schema: r#"{"name":"string?"}"#, executable: true },
        ToolSpec { id: "Sleep", group: "control", use_text: "Wait briefly before continuing.", input_schema: r#"{"seconds":"number?"}"#, executable: true },
        ToolSpec { id: "TaskCreate", group: "delegation", use_text: "Create a bounded task packet for later execution.", input_schema: r#"{"title":"string","details":"string?"}"#, executable: true },
        ToolSpec { id: "Task", group: "delegation", use_text: "Delegate a bounded task to a subagent.", input_schema: r#"{"agent":"string","task":"string"}"#, executable: true },
        ToolSpec { id: "TeamDelete", group: "coordination", use_text: "Delete a teammate handle from the current team registry.", input_schema: r#"{"name":"string"}"#, executable: true },
        ToolSpec { id: "TeammateTool", group: "coordination", use_text: "Register or inspect a teammate in the coordination layer.", input_schema: r#"{"name":"string?"}"#, executable: true },
        ToolSpec { id: "TodoWrite", group: "planning", use_text: "Replace the session todo list.", input_schema: r#"{"items":"string[]"}"#, executable: true },
        ToolSpec { id: "ToolSearch", group: "meta", use_text: "Search the known tool catalog by name or purpose.", input_schema: r#"{"query":"string"}"#, executable: true },
        ToolSpec { id: "WebFetch", group: "web", use_text: "Fetch a public web page by URL.", input_schema: r#"{"url":"string"}"#, executable: true },
        ToolSpec { id: "WebSearch", group: "web", use_text: "Run a simple web search and return result snippets.", input_schema: r#"{"query":"string"}"#, executable: true },
        ToolSpec { id: "Write", group: "files", use_text: "Write or overwrite a text file inside the workspace.", input_schema: r#"{"path":"string","content":"string"}"#, executable: true },
    ]
}

pub fn executable_tools() -> Vec<ToolSpec> {
    tool_catalog().into_iter().filter(|tool| tool.executable).collect()
}

fn run_lsp(input: &Value, runtime: &RuntimeConfig) -> Result<String> {
    let query = get_string(input, "query")?;
    let scoped_path = input
        .get("path")
        .and_then(|value| value.as_str())
        .map(|value| runtime.workspace_root.join(value));
    let mut results = Vec::new();

    let walker = if let Some(path) = scoped_path {
        WalkDir::new(path)
    } else {
        WalkDir::new(&runtime.workspace_root)
    };

    for entry in walker.into_iter().filter_map(|entry| entry.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(&runtime.workspace_root)
            .unwrap_or(entry.path())
            .to_string_lossy()
            .to_string();
        if let Ok(content) = fs::read_to_string(entry.path()) {
            for (index, line) in content.lines().enumerate() {
                if line.contains(&query) {
                    results.push(format!("{}:{}: {}", relative, index + 1, line.trim()));
                    if results.len() >= 100 {
                        break;
                    }
                }
            }
        }
        if results.len() >= 100 {
            break;
        }
    }

    Ok(if results.is_empty() {
        "No symbol-like matches found.".to_string()
    } else {
        results.join("\n")
    })
}

fn encode_query(query: &str) -> String {
    query
        .replace('%', "%25")
        .replace(' ', "+")
        .replace('"', "%22")
        .replace('#', "%23")
        .replace('&', "%26")
        .replace('?', "%3F")
}

pub async fn run_tool(tool: &str, input: &Value, runtime: &RuntimeConfig, context: &mut ToolContext) -> Result<String> {
    match tool {
        "ReadFile" => {
            let path = get_string(input, "path")?;
            let file_path = resolve_within_root(&runtime.workspace_root, &path)?;
            let content = fs::read_to_string(file_path).with_context(|| format!("failed to read {}", path))?;
            Ok(truncate(content, runtime.max_tool_output_chars))
        }
        "Write" => {
            let path = get_string(input, "path")?;
            let content = get_string(input, "content")?;
            let file_path = runtime.workspace_root.join(&path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&file_path, content)?;
            Ok(format!("Wrote {}", path))
        }
        "Edit" => {
            let path = get_string(input, "path")?;
            let old_string = get_string(input, "old_string")?;
            let new_string = get_string(input, "new_string")?;
            let replace_all = input.get("all").and_then(|value| value.as_bool()).unwrap_or(false);
            let file_path = resolve_within_root(&runtime.workspace_root, &path)?;
            let content = fs::read_to_string(&file_path)?;
            if !content.contains(&old_string) {
                bail!("old_string not found in {}", path);
            }
            let updated = if replace_all {
                content.replace(&old_string, &new_string)
            } else {
                content.replacen(&old_string, &new_string, 1)
            };
            fs::write(&file_path, updated)?;
            Ok(format!("Edited {}", path))
        }
        "NotebookEdit" => {
            let path = get_string(input, "path")?;
            let content = get_string(input, "content")?;
            let file_path = runtime.workspace_root.join(&path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&file_path, content)?;
            Ok(format!("Updated notebook-like file {}", path))
        }
        "Glob" => {
            let pattern = get_string(input, "pattern")?;
            let mut builder = GlobSetBuilder::new();
            builder.add(Glob::new(&pattern)?);
            let matcher = builder.build()?;
            let matches = WalkDir::new(&runtime.workspace_root)
                .into_iter()
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.file_type().is_file())
                .filter_map(|entry| {
                    entry
                        .path()
                        .strip_prefix(&runtime.workspace_root)
                        .ok()
                        .map(|p| p.to_string_lossy().to_string())
                })
                .filter(|relative| {
                    matcher.is_match(relative)
                        || matcher.is_match(Path::new(relative).file_name().unwrap_or_default())
                })
                .take(200)
                .collect::<Vec<_>>();
            Ok(if matches.is_empty() {
                "No files matched.".to_string()
            } else {
                matches.join("\n")
            })
        }
        "Grep" => {
            let query = get_string(input, "query")?;
            let mut results = Vec::new();
            for entry in WalkDir::new(&runtime.workspace_root).into_iter().filter_map(|entry| entry.ok()) {
                if !entry.file_type().is_file() {
                    continue;
                }
                let relative = entry
                    .path()
                    .strip_prefix(&runtime.workspace_root)
                    .unwrap_or(entry.path())
                    .to_string_lossy()
                    .to_string();
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    for (index, line) in content.lines().enumerate() {
                        if line.contains(&query) {
                            results.push(format!("{}:{}: {}", relative, index + 1, line));
                            if results.len() >= 200 {
                                break;
                            }
                        }
                    }
                }
                if results.len() >= 200 {
                    break;
                }
            }
            Ok(if results.is_empty() {
                "No matches found.".to_string()
            } else {
                results.join("\n")
            })
        }
        "LSP" => run_lsp(input, runtime),
        "Bash" => {
            if !runtime.allow_bash {
                bail!("Bash is disabled by configuration");
            }
            let command = get_string(input, "command")?;
            let output = Command::new("/bin/zsh")
                .arg("-lc")
                .arg(&command)
                .current_dir(&runtime.workspace_root)
                .output()?;
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            Ok(truncate(
                [stdout, stderr]
                    .into_iter()
                    .filter(|part| !part.is_empty())
                    .collect::<Vec<_>>()
                    .join("\n"),
                runtime.max_tool_output_chars,
            ))
        }
        "TodoWrite" => {
            context.todos = input
                .get("items")
                .and_then(|value| value.as_array())
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|item| item.as_str().map(|value| value.to_string()))
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();
            Ok(format!(
                "Updated todos:\n{}",
                context
                    .todos
                    .iter()
                    .map(|item| format!("- {}", item))
                    .collect::<Vec<_>>()
                    .join("\n")
            ))
        }
        "AskUserQuestion" => {
            if !context.interactive {
                bail!("AskUserQuestion is only available in interactive mode");
            }
            let question = get_string(input, "question")?;
            print!("{}\n> ", question);
            io::stdout().flush()?;
            let mut answer = String::new();
            io::stdin().read_line(&mut answer)?;
            Ok(format!("User answer: {}", answer.trim()))
        }
        "WebFetch" => {
            let url = get_string(input, "url")?;
            let response = reqwest::get(url).await?;
            let text = response.text().await?;
            Ok(truncate(text, runtime.max_tool_output_chars))
        }
        "WebSearch" => {
            let query = get_string(input, "query")?;
            let url = format!("https://duckduckgo.com/html/?q={}", encode_query(&query));
            let text = reqwest::get(url).await?.text().await?;
            Ok(truncate(text, runtime.max_tool_output_chars))
        }
        "ToolSearch" => {
            let query = get_string(input, "query")?.to_lowercase();
            let matches = tool_catalog()
                .into_iter()
                .filter(|tool| {
                    tool.id.to_lowercase().contains(&query)
                        || tool.use_text.to_lowercase().contains(&query)
                        || tool.group.to_lowercase().contains(&query)
                })
                .map(|tool| {
                    format!(
                        "{} [{}] - {}",
                        tool.id,
                        if tool.executable { "executable" } else { "catalog-only" },
                        tool.use_text
                    )
                })
                .collect::<Vec<_>>();
            Ok(if matches.is_empty() {
                "No tools matched.".to_string()
            } else {
                matches.join("\n")
            })
        }
        "Skill" => {
            let requested = input.get("name").and_then(|value| value.as_str()).unwrap_or("all");
            let catalog = [
                json!({"id": "remember-skill", "summary": "Propose durable memory updates from recurring patterns."}),
                json!({"id": "skillify-current-session", "summary": "Turn the current interaction into a reusable workflow."}),
                json!({"id": "claude-guide-agent", "summary": "Explain the Claude Code-like runtime and its moving parts."}),
            ];
            let filtered = if requested == "all" {
                catalog.to_vec()
            } else {
                catalog
                    .into_iter()
                    .filter(|item| item["id"].as_str() == Some(requested))
                    .collect::<Vec<_>>()
            };
            Ok(serde_json::to_string_pretty(&filtered)?)
        }
        "EnterPlanMode" => {
            context.plan_mode = true;
            Ok("Plan mode enabled for this session.".to_string())
        }
        "ExitPlanMode" => {
            context.plan_mode = false;
            Ok("Plan mode disabled for this session.".to_string())
        }
        "TaskCreate" => {
            let title = get_string(input, "title")?;
            let details = input.get("details").and_then(|value| value.as_str()).unwrap_or("");
            let rendered = if details.is_empty() {
                title
            } else {
                format!("{}: {}", title, details)
            };
            context.todos.push(rendered.clone());
            Ok(format!("Created task packet: {}", rendered))
        }
        "SendMessageTool" => {
            let to = get_string(input, "to")?;
            let message = get_string(input, "message")?;
            context.messages.push(format!("{} <= {}", to, message));
            Ok(format!("Sent message to {}", to))
        }
        "TeammateTool" => {
            if let Some(name) = input.get("name").and_then(|value| value.as_str()) {
                if !context.teammates.iter().any(|item| item == name) {
                    context.teammates.push(name.to_string());
                }
                Ok(format!("Active teammates: {}", context.teammates.join(", ")))
            } else if context.teammates.is_empty() {
                Ok("No teammates registered.".to_string())
            } else {
                Ok(format!("Active teammates: {}", context.teammates.join(", ")))
            }
        }
        "TeamDelete" => {
            let name = get_string(input, "name")?;
            context.teammates.retain(|item| item != &name);
            Ok(format!("Removed teammate {}", name))
        }
        "Sleep" => {
            let seconds = input.get("seconds").and_then(|value| value.as_f64()).unwrap_or(1.0).clamp(0.0, 2.0);
            thread::sleep(Duration::from_millis((seconds * 1000.0) as u64));
            Ok(format!("Slept for {:.1} seconds", seconds))
        }
        "Computer" => {
            let action = input.get("action").and_then(|value| value.as_str()).unwrap_or("inspect");
            let target = input.get("target").and_then(|value| value.as_str()).unwrap_or("screen");
            Ok(format!("Computer tool is cataloged but not implemented in this Rust runtime. Requested action: {} on {}", action, target))
        }
        other => bail!("unknown or unimplemented tool: {}", other),
    }
}
