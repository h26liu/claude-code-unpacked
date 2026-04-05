async function requestJson(url, init) {
  const response = await fetch(url, init);
  const text = await response.text();

  if (!response.ok) {
    throw new Error(`HTTP ${response.status} from ${url}\n${text}`);
  }

  return JSON.parse(text);
}

function joinUrl(baseUrl, suffix) {
  return `${baseUrl.replace(/\/+$/, "")}${suffix}`;
}

function extractSystem(messages) {
  return messages.filter((message) => message.role === "system").map((message) => message.content).join("\n\n");
}

function extractNonSystem(messages) {
  return messages.filter((message) => message.role !== "system");
}

function buildMockResponse(messages) {
  const lastMessage = messages[messages.length - 1]?.content || "";
  const sawToolResult = messages.some((message) => /Tool result:/i.test(message.content));

  if (!sawToolResult && /readme|file|inspect|understand|look at/i.test(lastMessage)) {
    return JSON.stringify({
      type: "tool_call",
      tool: "Glob",
      input: { pattern: "*.md" },
      summary: "List markdown files before responding."
    });
  }

  return JSON.stringify({
    type: "final",
    message: "Mock provider final answer. Configure a real provider in cli-rebuild.config.json or .env to use an external model.",
    summary: "Completed with the built-in mock provider."
  });
}

async function callMock({ messages }) {
  return buildMockResponse(messages);
}

async function callOpenAICompatible({ provider, messages }) {
  const body = {
    model: provider.model,
    temperature: provider.temperature,
    max_tokens: provider.maxOutputTokens,
    messages
  };

  const headers = {
    "content-type": "application/json",
    authorization: `Bearer ${provider.apiKey}`
  };

  const data = await requestJson(joinUrl(provider.baseUrl, "/chat/completions"), {
    method: "POST",
    headers,
    body: JSON.stringify(body)
  });

  return data.choices?.[0]?.message?.content || "";
}

async function callOpenRouter({ provider, messages }) {
  const body = {
    model: provider.model,
    temperature: provider.temperature,
    max_tokens: provider.maxOutputTokens,
    messages
  };

  const headers = {
    "content-type": "application/json",
    authorization: `Bearer ${provider.apiKey}`
  };

  const data = await requestJson(joinUrl(provider.baseUrl, "/chat/completions"), {
    method: "POST",
    headers,
    body: JSON.stringify(body)
  });

  return data.choices?.[0]?.message?.content || "";
}

async function callAnthropic({ provider, messages }) {
  const body = {
    model: provider.model,
    temperature: provider.temperature,
    max_tokens: provider.maxOutputTokens,
    system: extractSystem(messages),
    messages: extractNonSystem(messages).map((message) => ({
      role: message.role === "assistant" ? "assistant" : "user",
      content: message.content
    }))
  };

  const headers = {
    "content-type": "application/json",
    "x-api-key": provider.apiKey,
    "anthropic-version": "2023-06-01"
  };

  const data = await requestJson(joinUrl(provider.baseUrl, "/messages"), {
    method: "POST",
    headers,
    body: JSON.stringify(body)
  });

  return data.content?.map((item) => item.text || "").join("\n") || "";
}

export function normalizeProvider(provider) {
  const normalized = { ...provider };

  if (normalized.type === "openai" && !normalized.baseUrl) {
    normalized.baseUrl = "https://api.openai.com/v1";
  }

  if (normalized.type === "openrouter" && !normalized.baseUrl) {
    normalized.baseUrl = "https://openrouter.ai/api/v1";
  }

  if (normalized.type === "openai-compatible" && !normalized.baseUrl) {
    throw new Error("openai-compatible provider requires `baseUrl`.");
  }

  if (normalized.type === "anthropic" && !normalized.baseUrl) {
    normalized.baseUrl = "https://api.anthropic.com/v1";
  }

  return normalized;
}

export async function callModel({ provider, messages }) {
  const normalized = normalizeProvider(provider);

  if (normalized.type === "mock") {
    return callMock({ provider: normalized, messages });
  }

  if (!normalized.apiKey) {
    throw new Error(`Provider ${normalized.type} requires an API key.`);
  }

  if (normalized.type === "openai" || normalized.type === "openai-compatible") {
    return callOpenAICompatible({ provider: normalized, messages });
  }

  if (normalized.type === "openrouter") {
    return callOpenRouter({ provider: normalized, messages });
  }

  if (normalized.type === "anthropic") {
    return callAnthropic({ provider: normalized, messages });
  }

  throw new Error(`Unsupported provider type: ${normalized.type}`);
}
