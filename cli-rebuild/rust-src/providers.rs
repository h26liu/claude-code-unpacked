use anyhow::{Context, Result, bail};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE, HeaderMap, HeaderValue};
use serde_json::{Value, json};

use crate::config::ProviderConfig;

#[derive(Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

fn join_url(base_url: &str, suffix: &str) -> String {
    format!("{}{}", base_url.trim_end_matches('/'), suffix)
}

fn extract_system(messages: &[Message]) -> String {
    messages
        .iter()
        .filter(|message| message.role == "system")
        .map(|message| message.content.clone())
        .collect::<Vec<_>>()
        .join("\n\n")
}

fn extract_non_system(messages: &[Message]) -> Vec<Value> {
    messages
        .iter()
        .filter(|message| message.role != "system")
        .map(|message| json!({
            "role": if message.role == "assistant" { "assistant" } else { "user" },
            "content": message.content,
        }))
        .collect()
}

fn build_mock_response(messages: &[Message]) -> String {
    let last_message = messages.last().map(|message| message.content.as_str()).unwrap_or("");
    let saw_tool_result = messages.iter().any(|message| message.content.contains("Tool result:"));

    if !saw_tool_result && ["readme", "file", "inspect", "understand", "look at"].iter().any(|term| last_message.to_lowercase().contains(term)) {
        return json!({
            "type": "tool_call",
            "tool": "Glob",
            "input": { "pattern": "*.md" },
            "summary": "List markdown files before responding."
        }).to_string();
    }

    json!({
        "type": "final",
        "message": "Mock provider final answer. Configure a real provider in cli-rebuild.config.toml or .env to use an external model.",
        "summary": "Completed with the built-in mock provider."
    }).to_string()
}

async fn request_json(url: String, headers: HeaderMap, body: Value) -> Result<Value> {
    let client = reqwest::Client::new();
    let response = client.post(url.clone()).headers(headers).json(&body).send().await?;
    let status = response.status();
    let text = response.text().await?;

    if !status.is_success() {
        bail!("HTTP {} from {}\n{}", status, url, text);
    }

    serde_json::from_str(&text).context("failed to parse provider JSON response")
}

pub async fn call_model(provider: &ProviderConfig, messages: &[Message]) -> Result<String> {
    match provider.r#type.as_str() {
        "mock" => Ok(build_mock_response(messages)),
        "openai" => call_openai_compatible(if provider.base_url.is_empty() { "https://api.openai.com/v1" } else { &provider.base_url }, provider, messages).await,
        "openai-compatible" => {
            if provider.base_url.is_empty() { bail!("openai-compatible provider requires base_url"); }
            call_openai_compatible(&provider.base_url, provider, messages).await
        }
        "openrouter" => call_openai_compatible(if provider.base_url.is_empty() { "https://openrouter.ai/api/v1" } else { &provider.base_url }, provider, messages).await,
        "anthropic" => call_anthropic(if provider.base_url.is_empty() { "https://api.anthropic.com/v1" } else { &provider.base_url }, provider, messages).await,
        other => bail!("unsupported provider type: {}", other),
    }
}

async fn call_openai_compatible(base_url: &str, provider: &ProviderConfig, messages: &[Message]) -> Result<String> {
    if provider.api_key.is_empty() {
        bail!("provider {} requires api_key", provider.r#type);
    }

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&format!("Bearer {}", provider.api_key))?);

    let body = json!({
        "model": provider.model,
        "temperature": provider.temperature,
        "max_tokens": provider.max_output_tokens,
        "messages": messages.iter().map(|message| json!({ "role": message.role, "content": message.content })).collect::<Vec<_>>()
    });

    let data = request_json(join_url(base_url, "/chat/completions"), headers, body).await?;
    Ok(data["choices"][0]["message"]["content"].as_str().unwrap_or_default().to_string())
}

async fn call_anthropic(base_url: &str, provider: &ProviderConfig, messages: &[Message]) -> Result<String> {
    if provider.api_key.is_empty() {
        bail!("provider anthropic requires api_key");
    }

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("x-api-key", HeaderValue::from_str(&provider.api_key)?);
    headers.insert("anthropic-version", HeaderValue::from_static("2023-06-01"));

    let body = json!({
        "model": provider.model,
        "temperature": provider.temperature,
        "max_tokens": provider.max_output_tokens,
        "system": extract_system(messages),
        "messages": extract_non_system(messages),
    });

    let data = request_json(join_url(base_url, "/messages"), headers, body).await?;
    let text = data["content"].as_array().cloned().unwrap_or_default().into_iter().filter_map(|item| item["text"].as_str().map(|s| s.to_string())).collect::<Vec<_>>().join("\n");
    Ok(text)
}
