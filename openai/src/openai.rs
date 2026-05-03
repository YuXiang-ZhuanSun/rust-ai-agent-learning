use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;

#[derive(Debug, Clone)]
pub struct OpenAiClient {
    http: Client,
    api_key: String,
    base_url: String,
    model: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    System,
    User,
    Assistant,
    Tool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: Role,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub function: ToolFunctionCall,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolFunctionCall {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ToolSpec {
    #[serde(rename = "type")]
    kind: &'static str,
    function: FunctionSpec,
}

#[derive(Debug, Clone, Serialize)]
struct FunctionSpec {
    name: String,
    description: String,
    parameters: Value,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ChatMessage,
    finish_reason: Option<String>,
}

impl OpenAiClient {
    pub fn from_env() -> Result<Self> {
        let api_key = env::var("OPENAI_API_KEY").context("OPENAI_API_KEY is not set")?;
        let base_url = env::var("OPENAI_BASE_URL")
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
        let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-4o-mini".to_string());

        Ok(Self {
            http: Client::new(),
            api_key,
            base_url: base_url.trim_end_matches('/').to_string(),
            model,
        })
    }

    pub async fn chat(&self, messages: &[ChatMessage], tools: &[ToolSpec]) -> Result<ChatMessage> {
        let body = json!({
            "model": self.model,
            "messages": messages,
            "tools": tools,
        });

        let response = self
            .http
            .post(format!("{}/chat/completions", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await
            .context("failed to send chat completion request")?;

        let status = response.status();
        let text = response.text().await.context("failed to read response body")?;
        if !status.is_success() {
            return Err(anyhow!("OpenAI API returned {status}: {text}"));
        }

        let parsed: ChatResponse =
            serde_json::from_str(&text).context("failed to parse chat completion response")?;
        let choice = parsed
            .choices
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("chat completion returned no choices"))?;

        let _finish_reason = choice.finish_reason;
        Ok(choice.message)
    }
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self::new(Role::System, content)
    }

    pub fn user(content: impl Into<String>) -> Self {
        Self::new(Role::User, content)
    }

    pub fn assistant(content: impl Into<String>) -> Self {
        Self::new(Role::Assistant, content)
    }

    pub fn tool(tool_call_id: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: Role::Tool,
            content: Some(content.into()),
            tool_call_id: Some(tool_call_id.into()),
            tool_calls: None,
        }
    }

    fn new(role: Role, content: impl Into<String>) -> Self {
        Self {
            role,
            content: Some(content.into()),
            tool_call_id: None,
            tool_calls: None,
        }
    }

    pub fn text(&self) -> &str {
        self.content.as_deref().unwrap_or("")
    }
}

impl ToolSpec {
    pub fn new(name: &str, description: &str, parameters: Value) -> Self {
        Self {
            kind: "function",
            function: FunctionSpec {
                name: name.to_string(),
                description: description.to_string(),
                parameters,
            },
        }
    }

    pub fn object(name: &str, description: &str, properties: Value, required: &[&str]) -> Self {
        Self::new(
            name,
            description,
            json!({
                "type": "object",
                "properties": properties,
                "required": required,
                "additionalProperties": false,
            }),
        )
    }
}

