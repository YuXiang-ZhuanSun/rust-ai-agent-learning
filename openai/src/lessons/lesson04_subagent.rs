use crate::lessons::lesson02_tools::ToolRegistry;
use crate::openai::{ChatMessage, OpenAiClient};
use anyhow::Result;
use std::path::PathBuf;

pub struct Subagent {
    client: OpenAiClient,
    tools: ToolRegistry,
}

impl Subagent {
    pub fn new(client: OpenAiClient, workspace: PathBuf) -> Self {
        Self {
            client,
            tools: ToolRegistry::new(workspace),
        }
    }

    pub async fn run(&self, prompt: &str, agent_type: &str) -> Result<String> {
        let mut messages = vec![
            ChatMessage::system(format!(
                "You are a focused {agent_type} subagent. Work independently and return a short summary."
            )),
            ChatMessage::user(prompt),
        ];

        for _ in 0..12 {
            let reply = self.client.chat(&messages, &self.tools.specs()).await?;
            if let Some(calls) = &reply.tool_calls {
                messages.push(reply.clone());
                for call in calls {
                    let args = serde_json::from_str(&call.function.arguments)?;
                    let output = self.tools.execute(&call.function.name, args)?;
                    messages.push(ChatMessage::tool(&call.id, output));
                }
            } else {
                return Ok(reply.text().to_string());
            }
        }
        Ok("subagent stopped after the safety round limit".to_string())
    }
}

