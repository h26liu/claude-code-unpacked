use std::{collections::HashMap, env, fs, path::{Path, PathBuf}};

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub r#type: String,
    pub model: String,
    pub base_url: String,
    pub api_key: String,
    pub api_key_env: String,
    pub temperature: f32,
    pub max_output_tokens: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub workspace_root: PathBuf,
    pub max_turns: usize,
    pub max_tool_output_chars: usize,
    pub allow_bash: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionConfig {
    pub scenario: String,
    pub plan_mode: bool,
    pub memory: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HooksConfig {
    pub pre_tool: Vec<String>,
    pub post_tool: Vec<String>,
    pub on_final: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub provider: ProviderConfig,
    pub runtime: RuntimeConfig,
    pub session: SessionConfig,
    pub hooks: HooksConfig,
}

#[derive(Debug, Clone)]
pub struct LoadedConfig {
    pub project_dir: PathBuf,
    pub config_path: PathBuf,
    pub env_path: PathBuf,
    pub config: Config,
}

#[derive(Debug, Default, Deserialize)]
struct PartialProviderConfig {
    r#type: Option<String>,
    model: Option<String>,
    base_url: Option<String>,
    api_key: Option<String>,
    api_key_env: Option<String>,
    temperature: Option<f32>,
    max_output_tokens: Option<u32>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialRuntimeConfig {
    workspace_root: Option<PathBuf>,
    max_turns: Option<usize>,
    max_tool_output_chars: Option<usize>,
    allow_bash: Option<bool>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialSessionConfig {
    scenario: Option<String>,
    plan_mode: Option<bool>,
    memory: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialHooksConfig {
    pre_tool: Option<Vec<String>>,
    post_tool: Option<Vec<String>>,
    on_final: Option<Vec<String>>,
}

#[derive(Debug, Default, Deserialize)]
struct PartialConfig {
    provider: Option<PartialProviderConfig>,
    runtime: Option<PartialRuntimeConfig>,
    session: Option<PartialSessionConfig>,
    hooks: Option<PartialHooksConfig>,
}

fn default_config(project_dir: &Path) -> Config {
    Config {
        provider: ProviderConfig {
            r#type: "mock".to_string(),
            model: "mock-claude-code".to_string(),
            base_url: String::new(),
            api_key: String::new(),
            api_key_env: "CLI_REBUILD_API_KEY".to_string(),
            temperature: 0.2,
            max_output_tokens: 1200,
        },
        runtime: RuntimeConfig {
            workspace_root: project_dir.to_path_buf(),
            max_turns: 10,
            max_tool_output_chars: 12_000,
            allow_bash: true,
        },
        session: SessionConfig {
            scenario: "default".to_string(),
            plan_mode: false,
            memory: Vec::new(),
        },
        hooks: HooksConfig::default(),
    }
}

fn apply_partial(config: &mut Config, partial: PartialConfig) {
    if let Some(provider) = partial.provider {
        if let Some(value) = provider.r#type { config.provider.r#type = value; }
        if let Some(value) = provider.model { config.provider.model = value; }
        if let Some(value) = provider.base_url { config.provider.base_url = value; }
        if let Some(value) = provider.api_key { config.provider.api_key = value; }
        if let Some(value) = provider.api_key_env { config.provider.api_key_env = value; }
        if let Some(value) = provider.temperature { config.provider.temperature = value; }
        if let Some(value) = provider.max_output_tokens { config.provider.max_output_tokens = value; }
    }

    if let Some(runtime) = partial.runtime {
        if let Some(value) = runtime.workspace_root { config.runtime.workspace_root = value; }
        if let Some(value) = runtime.max_turns { config.runtime.max_turns = value; }
        if let Some(value) = runtime.max_tool_output_chars { config.runtime.max_tool_output_chars = value; }
        if let Some(value) = runtime.allow_bash { config.runtime.allow_bash = value; }
    }

    if let Some(session) = partial.session {
        if let Some(value) = session.scenario { config.session.scenario = value; }
        if let Some(value) = session.plan_mode { config.session.plan_mode = value; }
        if let Some(value) = session.memory { config.session.memory = value; }
    }

    if let Some(hooks) = partial.hooks {
        if let Some(value) = hooks.pre_tool { config.hooks.pre_tool = value; }
        if let Some(value) = hooks.post_tool { config.hooks.post_tool = value; }
        if let Some(value) = hooks.on_final { config.hooks.on_final = value; }
    }
}

fn apply_env(config: &mut Config, env_map: &HashMap<String, String>) {
    if let Some(value) = env_map.get("CLI_REBUILD_PROVIDER") { config.provider.r#type = value.clone(); }
    if let Some(value) = env_map.get("CLI_REBUILD_MODEL") { config.provider.model = value.clone(); }
    if let Some(value) = env_map.get("CLI_REBUILD_BASE_URL") { config.provider.base_url = value.clone(); }
    if let Some(value) = env_map.get("CLI_REBUILD_API_KEY") { config.provider.api_key = value.clone(); }
    if let Some(value) = env_map.get("CLI_REBUILD_API_KEY_ENV") { config.provider.api_key_env = value.clone(); }
    if let Some(value) = env_map.get("CLI_REBUILD_TEMPERATURE") { if let Ok(parsed) = value.parse() { config.provider.temperature = parsed; } }
    if let Some(value) = env_map.get("CLI_REBUILD_MAX_OUTPUT_TOKENS") { if let Ok(parsed) = value.parse() { config.provider.max_output_tokens = parsed; } }
    if let Some(value) = env_map.get("CLI_REBUILD_WORKSPACE_ROOT") { config.runtime.workspace_root = PathBuf::from(value); }
    if let Some(value) = env_map.get("CLI_REBUILD_MAX_TURNS") { if let Ok(parsed) = value.parse() { config.runtime.max_turns = parsed; } }
    if let Some(value) = env_map.get("CLI_REBUILD_MAX_TOOL_OUTPUT_CHARS") { if let Ok(parsed) = value.parse() { config.runtime.max_tool_output_chars = parsed; } }
    if let Some(value) = env_map.get("CLI_REBUILD_ALLOW_BASH") { config.runtime.allow_bash = value != "false"; }
    if let Some(value) = env_map.get("CLI_REBUILD_SCENARIO") { config.session.scenario = value.clone(); }
    if let Some(value) = env_map.get("CLI_REBUILD_PLAN_MODE") { config.session.plan_mode = value == "true"; }

    if let Some(value) = env_map.get("CLI_REBUILD_PRE_TOOL_HOOK") { config.hooks.pre_tool = vec![value.clone()]; }
    if let Some(value) = env_map.get("CLI_REBUILD_POST_TOOL_HOOK") { config.hooks.post_tool = vec![value.clone()]; }
    if let Some(value) = env_map.get("CLI_REBUILD_ON_FINAL_HOOK") { config.hooks.on_final = vec![value.clone()]; }

    if config.provider.api_key.is_empty() {
        if let Some(value) = env_map.get(&config.provider.api_key_env) {
            config.provider.api_key = value.clone();
        }
    }
}

fn read_dotenv(path: &Path) -> Result<HashMap<String, String>> {
    let mut env_map = HashMap::new();

    if !path.exists() {
        return Ok(env_map);
    }

    let content = fs::read_to_string(path).with_context(|| format!("failed to read {}", path.display()))?;
    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        if let Some((key, value)) = line.split_once('=') {
            env_map.insert(key.trim().to_string(), value.trim().trim_matches('"').trim_matches('\'').to_string());
        }
    }

    Ok(env_map)
}

pub fn load_config() -> Result<LoadedConfig> {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let config_path = project_dir.join("cli-rebuild.config.toml");
    let env_path = project_dir.join(".env");
    let mut config = default_config(&project_dir);

    if config_path.exists() {
        let content = fs::read_to_string(&config_path).with_context(|| format!("failed to read {}", config_path.display()))?;
        let partial: PartialConfig = toml::from_str(&content).context("failed to parse cli-rebuild.config.toml")?;
        apply_partial(&mut config, partial);
    }

    let mut env_map = read_dotenv(&env_path)?;
    for (key, value) in std::env::vars() {
        env_map.insert(key, value);
    }
    apply_env(&mut config, &env_map);

    if !config.runtime.workspace_root.is_absolute() {
        config.runtime.workspace_root = project_dir.join(&config.runtime.workspace_root);
    }

    Ok(LoadedConfig { project_dir, config_path, env_path, config })
}

fn redact_secret(value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }
    if value.len() <= 8 {
        return "********".to_string();
    }
    format!("{}...{}", &value[..4], &value[value.len() - 4..])
}

pub fn render_config_summary(loaded: &LoadedConfig) -> Result<String> {
    Ok(serde_json::to_string_pretty(&json!({
        "project_dir": loaded.project_dir,
        "config_path": loaded.config_path,
        "env_path": loaded.env_path,
        "provider": {
            "type": loaded.config.provider.r#type,
            "model": loaded.config.provider.model,
            "base_url": loaded.config.provider.base_url,
            "api_key": redact_secret(&loaded.config.provider.api_key),
            "api_key_env": loaded.config.provider.api_key_env,
            "temperature": loaded.config.provider.temperature,
            "max_output_tokens": loaded.config.provider.max_output_tokens,
        },
        "runtime": loaded.config.runtime,
        "session": loaded.config.session,
        "hooks": loaded.config.hooks,
    }))?)
}
