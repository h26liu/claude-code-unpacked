use std::{collections::HashMap, env, fs, path::{Path, PathBuf}};

use crate::catalog::{Agent, Catalog, Reminder, Scenario, Skill, Workflow, build_catalog};

#[derive(Debug, Clone)]
pub struct MemorySource {
    pub path: PathBuf,
    pub label: String,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct LoadedCommand {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct LoadedPrompt {
    pub id: String,
    pub title: String,
    pub body: String,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct LoadedAgent {
    pub id: String,
    pub title: String,
    pub summary: String,
    pub body: String,
    pub tools: Vec<String>,
    pub path: PathBuf,
}

#[derive(Debug, Clone)]
pub struct PreparedSession {
    pub catalog: Catalog,
    pub scenario: Scenario,
    pub workflow: Workflow,
    pub layers: Vec<crate::catalog::Layer>,
    pub reminders: Vec<Reminder>,
    pub skills: Vec<Skill>,
    pub memory: Vec<String>,
    pub memory_sources: Vec<MemorySource>,
    pub loaded_commands: Vec<LoadedCommand>,
    pub loaded_agents: Vec<LoadedAgent>,
    pub loaded_prompts: Vec<LoadedPrompt>,
    pub command_context: Vec<String>,
    pub agents: Vec<(String, Agent, String)>,
}

fn find_scenario<'a>(catalog: &'a Catalog, id: &str) -> Scenario {
    catalog.scenarios.iter().find(|scenario| scenario.id == id).cloned().unwrap_or_else(|| catalog.scenarios[0].clone())
}

fn find_agent(catalog: &Catalog, id: &str) -> Agent {
    catalog.agents.iter().find(|agent| agent.id == id).cloned().unwrap()
}

fn choose_workflow(catalog: &Catalog, task: &str, scenario_id: &str, plan_mode: bool) -> Workflow {
    let task_lower = task.to_lowercase();
    let mut matches = Vec::new();

    for workflow in &catalog.workflows {
        let matched = match workflow.id {
            "swarm-design" => ["swarm", "team", "multi-agent", "coordination"].iter().any(|term| task_lower.contains(term)),
            "review-swarm" => scenario_id == "review" || ["review", "diff", "regression", "bug", "test"].iter().any(|term| task_lower.contains(term)),
            "planning-delegation" => plan_mode,
            "implementation-default" => true,
            _ => false,
        };

        if matched {
            matches.push(workflow.clone());
        }
    }

    matches.sort_by_key(|workflow| -workflow.priority);
    matches.into_iter().next().unwrap_or_else(|| catalog.workflows[0].clone())
}

fn active_reminders(task: &str, scenario_id: &str, plan_mode: bool) -> Vec<Reminder> {
    let task_lower = task.to_lowercase();
    let mut reminders = Vec::new();

    if plan_mode {
        reminders.push(Reminder { id: "plan-mode", title: "Plan mode reminder", text: "Plan mode is active. Focus on decomposition, risks, and sequencing before execution." });
    }

    if scenario_id == "review" || ["review", "diff", "regression", "bug", "test"].iter().any(|term| task_lower.contains(term)) {
        reminders.push(Reminder { id: "review-mode", title: "Review reminder", text: "This looks like a review flow. Prioritize correctness, regressions, and missing tests." });
    }

    if ["doc", "readme", "explain", "writeup", "blog"].iter().any(|term| task_lower.contains(term)) {
        reminders.push(Reminder { id: "docs-mode", title: "Documentation reminder", text: "This looks documentation-heavy. Optimize for clarity, structure, and examples." });
    }

    reminders
}

fn active_skills(task: &str) -> Vec<Skill> {
    let task_lower = task.to_lowercase();
    let mut skills = Vec::new();

    if ["learn", "understand", "guide", "explain"].iter().any(|term| task_lower.contains(term)) {
        skills.push(Skill { id: "claude-guide-agent", title: "Claude guide agent", summary: "Helps users understand how to use Claude Code-like workflows and agents." });
    }

    if ["remember", "memory", "pattern", "learn"].iter().any(|term| task_lower.contains(term)) {
        skills.push(Skill { id: "remember-skill", title: "Remember skill", summary: "Reviews recurring patterns from the session and proposes durable memory updates." });
    }

    if ["skill", "workflow", "reusable", "template", "playbook"].iter().any(|term| task_lower.contains(term)) {
        skills.push(Skill { id: "skillify-current-session", title: "Skillify current session", summary: "Converts a useful interaction pattern into a reusable capability or workflow." });
    }

    skills
}

fn read_memory_source(path: &Path, label: &str) -> Option<MemorySource> {
    let content = fs::read_to_string(path).ok()?;
    if content.trim().is_empty() {
        return None;
    }
    Some(MemorySource {
        path: path.to_path_buf(),
        label: label.to_string(),
        content,
    })
}

fn markdown_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.filter_map(|entry| entry.ok()) {
            let path = entry.path();
            if path.is_file() {
                let is_md = path.extension().and_then(|value| value.to_str()).map(|value| value.eq_ignore_ascii_case("md")).unwrap_or(false);
                if is_md {
                    files.push(path);
                }
            }
        }
    }
    files.sort();
    files
}

fn load_memory_sources(workspace_root: &Path) -> Vec<MemorySource> {
    let mut sources = Vec::new();

    let project_candidates = [
        (workspace_root.join("CLAUDE.md"), "project memory"),
        (workspace_root.join(".claude/CLAUDE.md"), "project memory"),
        (workspace_root.join(".claude/memory.md"), "project memory"),
    ];

    for (path, label) in project_candidates {
        if let Some(source) = read_memory_source(&path, label) {
            sources.push(source);
        }
    }

    if let Some(home) = env::var_os("HOME") {
        let home = PathBuf::from(home);
        let user_candidates = [
            (home.join(".claude/CLAUDE.md"), "user memory"),
            (home.join(".config/claude-code/CLAUDE.md"), "user memory"),
            (home.join(".config/cli-rebuild/CLAUDE.md"), "user memory"),
        ];
        for (path, label) in user_candidates {
            if let Some(source) = read_memory_source(&path, label) {
                sources.push(source);
            }
        }
    }

    sources
}

fn slug_from_path(path: &Path) -> String {
    path.file_stem().and_then(|value| value.to_str()).unwrap_or("unknown").to_string()
}

fn parse_frontmatter(content: &str) -> (HashMap<String, String>, String) {
    let trimmed = content.trim_start();
    if !trimmed.starts_with("---\n") {
        return (HashMap::new(), content.to_string());
    }

    let mut lines = trimmed.lines();
    if lines.next() != Some("---") {
        return (HashMap::new(), content.to_string());
    }

    let mut header_lines = Vec::new();
    let mut body_lines = Vec::new();
    let mut in_header = true;
    for line in lines {
        if in_header && line.trim() == "---" {
            in_header = false;
            continue;
        }
        if in_header {
            header_lines.push(line.to_string());
        } else {
            body_lines.push(line.to_string());
        }
    }

    if in_header {
        return (HashMap::new(), content.to_string());
    }

    let mut map = HashMap::new();
    for line in header_lines {
        if let Some((key, value)) = line.split_once(':') {
            map.insert(key.trim().to_lowercase(), value.trim().trim_matches('"').trim_matches('\'').to_string());
        }
    }

    (map, body_lines.join("\n"))
}

fn heading_title(body: &str) -> Option<String> {
    body.lines()
        .map(str::trim)
        .find(|line| line.starts_with('#'))
        .map(|line| line.trim_start_matches('#').trim().to_string())
        .filter(|line| !line.is_empty())
}

fn body_summary(body: &str) -> String {
    body.lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('#'))
        .unwrap_or("")
        .to_string()
}

fn parse_tools(value: Option<&String>) -> Vec<String> {
    value
        .map(|raw| {
            raw.trim_matches('[')
                .trim_matches(']')
                .split(',')
                .map(|item| item.trim().trim_matches('"').trim_matches('\'').to_string())
                .filter(|item| !item.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

fn load_commands(workspace_root: &Path) -> Vec<LoadedCommand> {
    markdown_files(&workspace_root.join(".claude/commands"))
        .into_iter()
        .filter_map(|path| {
            let content = fs::read_to_string(&path).ok()?;
            let (frontmatter, body) = parse_frontmatter(&content);
            let id = frontmatter.get("id").cloned().unwrap_or_else(|| slug_from_path(&path));
            let title = frontmatter.get("title").cloned().or_else(|| heading_title(&body)).unwrap_or_else(|| id.clone());
            let summary = frontmatter.get("summary").cloned().unwrap_or_else(|| body_summary(&body));
            Some(LoadedCommand { id, title, summary, body: body.trim().to_string(), path })
        })
        .collect()
}

fn load_prompts(workspace_root: &Path) -> Vec<LoadedPrompt> {
    markdown_files(&workspace_root.join(".claude/prompts"))
        .into_iter()
        .filter_map(|path| {
            let content = fs::read_to_string(&path).ok()?;
            let (frontmatter, body) = parse_frontmatter(&content);
            let id = frontmatter.get("id").cloned().unwrap_or_else(|| slug_from_path(&path));
            let title = frontmatter.get("title").cloned().or_else(|| heading_title(&body)).unwrap_or_else(|| id.clone());
            Some(LoadedPrompt { id, title, body: body.trim().to_string(), path })
        })
        .collect()
}

fn load_agents(workspace_root: &Path) -> Vec<LoadedAgent> {
    markdown_files(&workspace_root.join(".claude/agents"))
        .into_iter()
        .filter_map(|path| {
            let content = fs::read_to_string(&path).ok()?;
            let (frontmatter, body) = parse_frontmatter(&content);
            let id = frontmatter.get("id").cloned().unwrap_or_else(|| slug_from_path(&path));
            let title = frontmatter.get("title").cloned().or_else(|| heading_title(&body)).unwrap_or_else(|| id.clone());
            let summary = frontmatter.get("summary").cloned().unwrap_or_else(|| body_summary(&body));
            let tools = parse_tools(frontmatter.get("tools"));
            Some(LoadedAgent { id, title, summary, body: body.trim().to_string(), tools, path })
        })
        .collect()
}

fn command_context(task: &str, loaded_commands: &[LoadedCommand]) -> Vec<String> {
    let first_token = task.split_whitespace().next().unwrap_or("");
    if !first_token.starts_with('/') {
        return Vec::new();
    }

    let command_id = first_token.trim_start_matches('/');
    loaded_commands
        .iter()
        .find(|command| command.id == command_id || slug_from_path(&command.path) == command_id)
        .map(|command| {
            vec![
                format!("Local command invoked: /{}", command_id),
                format!("Command title: {}", command.title),
                format!("Command summary: {}", command.summary),
                format!("Command body:\n{}", command.body),
            ]
        })
        .unwrap_or_default()
}

pub fn prepare_session(task: &str, scenario_id: &str, plan_mode: bool, memory: &[String], workspace_root: &Path) -> PreparedSession {
    let catalog = build_catalog();
    let scenario = find_scenario(&catalog, scenario_id);
    let workflow = choose_workflow(&catalog, task, scenario.id, plan_mode);
    let reminders = active_reminders(task, scenario.id, plan_mode);
    let skills = active_skills(task);
    let mut merged_memory = scenario.default_memory.iter().map(|item| item.to_string()).collect::<Vec<_>>();
    merged_memory.extend(memory.iter().cloned());

    let memory_sources = load_memory_sources(workspace_root);
    let loaded_commands = load_commands(workspace_root);
    let loaded_agents = load_agents(workspace_root);
    let loaded_prompts = load_prompts(workspace_root);
    let command_context = command_context(task, &loaded_commands);

    let agents = workflow.stages.iter().map(|stage| {
        (stage.title.to_string(), find_agent(&catalog, stage.agent), stage.note.to_string())
    }).collect::<Vec<_>>();

    PreparedSession {
        layers: catalog.layers.clone(),
        catalog,
        scenario,
        workflow,
        reminders,
        skills,
        memory: merged_memory,
        memory_sources,
        loaded_commands,
        loaded_agents,
        loaded_prompts,
        command_context,
        agents,
    }
}
