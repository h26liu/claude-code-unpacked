#[derive(Debug, Clone)]
pub struct Layer {
    pub id: &'static str,
    pub title: &'static str,
    pub text: &'static str,
}

#[derive(Debug, Clone)]
pub struct Reminder {
    pub id: &'static str,
    pub title: &'static str,
    pub text: &'static str,
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
}

#[derive(Debug, Clone)]
pub struct Agent {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub tools: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct WorkflowStage {
    pub title: &'static str,
    pub agent: &'static str,
    pub note: &'static str,
}

#[derive(Debug, Clone)]
pub struct Workflow {
    pub id: &'static str,
    pub title: &'static str,
    pub priority: i32,
    pub reason: &'static str,
    pub stages: Vec<WorkflowStage>,
}

#[derive(Debug, Clone)]
pub struct Scenario {
    pub id: &'static str,
    pub title: &'static str,
    pub default_memory: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub struct CommandDef {
    pub id: &'static str,
    pub title: &'static str,
    pub summary: &'static str,
}

#[derive(Debug, Clone)]
pub struct HookDef {
    pub id: &'static str,
    pub summary: &'static str,
}

#[derive(Debug, Clone)]
pub struct Catalog {
    pub layers: Vec<Layer>,
    pub agents: Vec<Agent>,
    pub skills: Vec<Skill>,
    pub scenarios: Vec<Scenario>,
    pub workflows: Vec<Workflow>,
    pub commands: Vec<CommandDef>,
    pub hooks: Vec<HookDef>,
}

pub fn build_catalog() -> Catalog {
    Catalog {
        layers: vec![
            Layer {
                id: "identity",
                title: "Identity and product role",
                text: "You are a terminal-native coding assistant operating inside a local workspace. Act as an execution-focused CLI, not a generic chatbot.",
            },
            Layer {
                id: "execution",
                title: "Execution policy",
                text: "Inspect first, act second, keep changes reviewable, and verify work when possible. Prefer grounded filesystem inspection over guessing.",
            },
            Layer {
                id: "tooling",
                title: "Tool policy",
                text: "Prefer search, file inspection, and bounded edits before broad or destructive operations. Never invent capabilities outside the visible tool surface.",
            },
            Layer {
                id: "delegation",
                title: "Delegation policy",
                text: "Use planning and task agents when specialization reduces confusion, narrows context, or improves reviewability.",
            },
            Layer {
                id: "memory",
                title: "Memory policy",
                text: "Treat project memory files, user memory, and session reminders as durable steering context. Reuse them when they are relevant.",
            },
            Layer {
                id: "commands-hooks",
                title: "Commands and hooks policy",
                text: "Respect slash-command style workflows and hook points around tool execution and final responses when they are configured.",
            },
        ],
        agents: vec![
            Agent { id: "explore", title: "Explore", description: "Gather context, inspect files, and map the workspace before acting.", tools: vec!["ReadFile", "Glob", "Grep", "Bash", "ToolSearch"] },
            Agent { id: "plan", title: "Plan", description: "Break down work, sequence steps, and identify risks or open questions.", tools: vec!["EnterPlanMode", "TodoWrite", "ReadFile", "Grep", "Bash", "TaskCreate", "Skill"] },
            Agent { id: "task", title: "Task", description: "Execute a bounded unit of work once the route is clear.", tools: vec!["ReadFile", "Edit", "Write", "Bash", "NotebookEdit", "Skill", "Task"] },
            Agent { id: "review", title: "Review", description: "Inspect changes for bugs, regressions, and test gaps.", tools: vec!["Bash", "ReadFile", "Grep", "LSP", "WebFetch", "WebSearch"] },
            Agent { id: "swarm-coordinator", title: "Swarm coordinator", description: "Coordinates multiple agents, assigns subtasks, and handles teammate messaging.", tools: vec!["SendMessageTool", "TaskCreate", "TeamDelete", "TeammateTool", "TodoWrite", "Task"] },
        ],
        skills: vec![
            Skill { id: "remember-skill", title: "Remember skill", summary: "Reviews recurring patterns from the session and proposes durable memory updates." },
            Skill { id: "skillify-current-session", title: "Skillify current session", summary: "Converts a useful interaction pattern into a reusable capability or workflow." },
            Skill { id: "claude-guide-agent", title: "Claude guide agent", summary: "Helps users understand how to use Claude Code-like workflows and agents." },
        ],
        scenarios: vec![
            Scenario { id: "default", title: "Default repo work", default_memory: vec!["Prefer small, reviewable changes.", "Explain architecture before large edits.", "Surface assumptions when reconstructing undocumented behavior."] },
            Scenario { id: "planning", title: "Architecture planning", default_memory: vec!["Prefer decomposition before implementation.", "Surface open questions and sequencing risks.", "Keep the design modular and easy to swap."] },
            Scenario { id: "review", title: "Code review", default_memory: vec!["Prioritize bugs over style comments.", "Look for behavior changes, regressions, and test gaps.", "Keep the review terse and evidence-driven."] },
        ],
        workflows: vec![
            Workflow {
                id: "swarm-design",
                title: "Swarm design flow",
                priority: 90,
                reason: "Coordination-heavy tasks benefit from an explicit orchestration stage before decomposition.",
                stages: vec![
                    WorkflowStage { title: "Coordination setup", agent: "swarm-coordinator", note: "Decide whether the problem should be split and how agents should communicate." },
                    WorkflowStage { title: "Planning", agent: "plan", note: "Break the coordinated work into bounded chunks." },
                    WorkflowStage { title: "Execution handoff", agent: "task", note: "Turn the design into concrete executable tasks." },
                ],
            },
            Workflow {
                id: "review-swarm",
                title: "Review swarm flow",
                priority: 80,
                reason: "Review work benefits from context gathering followed by a dedicated review pass.",
                stages: vec![
                    WorkflowStage { title: "Diff scan", agent: "explore", note: "Locate the changed files and summarize the behavioral surface." },
                    WorkflowStage { title: "Risk review", agent: "review", note: "Inspect correctness issues, regressions, and missing tests." },
                ],
            },
            Workflow {
                id: "planning-delegation",
                title: "Planning-first flow",
                priority: 40,
                reason: "Plan mode should front-load decomposition and delegation before execution.",
                stages: vec![
                    WorkflowStage { title: "Context scan", agent: "explore", note: "Inspect the workspace and collect just enough context to plan." },
                    WorkflowStage { title: "Decomposition", agent: "plan", note: "Break the work into stages, risks, and handoff points." },
                    WorkflowStage { title: "Execution handoff", agent: "task", note: "Turn the plan into a bounded implementation packet." },
                ],
            },
            Workflow {
                id: "implementation-default",
                title: "Default implementation flow",
                priority: 10,
                reason: "Implementation work usually benefits from quick exploration before execution.",
                stages: vec![
                    WorkflowStage { title: "Context scan", agent: "explore", note: "Map the workspace, find relevant files, and identify constraints." },
                    WorkflowStage { title: "Execution", agent: "task", note: "Apply a bounded edit once the target path is clear." },
                ],
            },
        ],
        commands: vec![
            CommandDef { id: "/review", title: "Review command", summary: "Bias the runtime toward review findings, regressions, and missing tests." },
            CommandDef { id: "/plan", title: "Plan command", summary: "Bias the runtime toward decomposition, sequencing, and execution planning." },
            CommandDef { id: "/agents", title: "Agents command", summary: "Inspect which specialized agents and workflows are available in the runtime." },
            CommandDef { id: "/memory", title: "Memory command", summary: "Inspect durable memory sources and what they contribute to the session." },
        ],
        hooks: vec![
            HookDef { id: "pre-tool", summary: "Runs before a tool call is executed." },
            HookDef { id: "post-tool", summary: "Runs after a tool call completes." },
            HookDef { id: "on-final", summary: "Runs before the final answer is returned to the user." },
        ],
    }
}
