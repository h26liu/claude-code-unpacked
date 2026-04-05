import fs from "node:fs/promises";
import path from "node:path";
import { fileURLToPath } from "node:url";

const DEFAULT_CONFIG = {
  provider: {
    type: "mock",
    model: "mock-claude-code",
    baseUrl: "",
    apiKey: "",
    apiKeyEnv: "CLI_REBUILD_API_KEY",
    temperature: 0.2,
    maxOutputTokens: 1200
  },
  runtime: {
    workspaceRoot: process.cwd(),
    maxTurns: 10,
    maxToolOutputChars: 12000,
    allowBash: true
  },
  session: {
    scenario: "default",
    planMode: false,
    memory: []
  }
};

function deepMerge(base, extra) {
  if (!extra || typeof extra !== "object" || Array.isArray(extra)) {
    return extra === undefined ? base : extra;
  }

  const result = { ...base };

  for (const [key, value] of Object.entries(extra)) {
    result[key] = key in base ? deepMerge(base[key], value) : value;
  }

  return result;
}

async function readJsonIfExists(filePath) {
  try {
    const content = await fs.readFile(filePath, "utf8");
    return JSON.parse(content);
  } catch (error) {
    if (error.code === "ENOENT") {
      return null;
    }

    throw error;
  }
}

async function readDotEnvIfExists(filePath) {
  try {
    const content = await fs.readFile(filePath, "utf8");
    const values = {};

    for (const rawLine of content.split(/\r?\n/)) {
      const line = rawLine.trim();
      if (!line || line.startsWith("#")) {
        continue;
      }

      const match = line.match(/^([A-Za-z_][A-Za-z0-9_]*)=(.*)$/);
      if (!match) {
        continue;
      }

      const [, key, rawValue] = match;
      values[key] = rawValue.replace(/^['"]|['"]$/g, "");
    }

    return values;
  } catch (error) {
    if (error.code === "ENOENT") {
      return {};
    }

    throw error;
  }
}

function envOverrides(env) {
  return {
    provider: {
      type: env.CLI_REBUILD_PROVIDER,
      model: env.CLI_REBUILD_MODEL,
      baseUrl: env.CLI_REBUILD_BASE_URL,
      apiKey: env.CLI_REBUILD_API_KEY,
      apiKeyEnv: env.CLI_REBUILD_API_KEY_ENV,
      temperature: env.CLI_REBUILD_TEMPERATURE ? Number(env.CLI_REBUILD_TEMPERATURE) : undefined,
      maxOutputTokens: env.CLI_REBUILD_MAX_OUTPUT_TOKENS ? Number(env.CLI_REBUILD_MAX_OUTPUT_TOKENS) : undefined
    },
    runtime: {
      workspaceRoot: env.CLI_REBUILD_WORKSPACE_ROOT,
      maxTurns: env.CLI_REBUILD_MAX_TURNS ? Number(env.CLI_REBUILD_MAX_TURNS) : undefined,
      maxToolOutputChars: env.CLI_REBUILD_MAX_TOOL_OUTPUT_CHARS
        ? Number(env.CLI_REBUILD_MAX_TOOL_OUTPUT_CHARS)
        : undefined,
      allowBash: env.CLI_REBUILD_ALLOW_BASH ? env.CLI_REBUILD_ALLOW_BASH !== "false" : undefined
    },
    session: {
      scenario: env.CLI_REBUILD_SCENARIO,
      planMode: env.CLI_REBUILD_PLAN_MODE ? env.CLI_REBUILD_PLAN_MODE === "true" : undefined
    }
  };
}

function maskSecret(value) {
  if (!value) {
    return "";
  }

  if (value.length <= 8) {
    return "********";
  }

  return `${value.slice(0, 4)}...${value.slice(-4)}`;
}

export async function loadConfig() {
  const rootDir = path.dirname(fileURLToPath(import.meta.url));
  const projectDir = path.join(rootDir, "..");
  const configPath = path.join(projectDir, "cli-rebuild.config.json");
  const envPath = path.join(projectDir, ".env");

  const configFile = (await readJsonIfExists(configPath)) || {};
  const envFile = await readDotEnvIfExists(envPath);
  const mergedEnv = { ...envFile, ...process.env };

  const config = deepMerge(DEFAULT_CONFIG, deepMerge(configFile, envOverrides(mergedEnv)));
  const apiKeyEnv = config.provider.apiKeyEnv || "CLI_REBUILD_API_KEY";

  if (!config.provider.apiKey && mergedEnv[apiKeyEnv]) {
    config.provider.apiKey = mergedEnv[apiKeyEnv];
  }

  if (!path.isAbsolute(config.runtime.workspaceRoot)) {
    config.runtime.workspaceRoot = path.resolve(projectDir, config.runtime.workspaceRoot);
  }

  return {
    projectDir,
    configPath,
    envPath,
    config
  };
}

export function renderConfigSummary(configInfo) {
  const { config, configPath, envPath } = configInfo;

  return JSON.stringify(
    {
      configPath,
      envPath,
      provider: {
        ...config.provider,
        apiKey: maskSecret(config.provider.apiKey)
      },
      runtime: config.runtime,
      session: config.session
    },
    null,
    2
  );
}
