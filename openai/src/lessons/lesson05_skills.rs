use crate::lessons::lesson02_tools::ToolRegistry;
use crate::openai::{ChatMessage, OpenAiClient, ToolSpec};
pub use crate::runtime::SkillLoader;
use anyhow::Result;
use serde_json::{json, Value};
use std::path::PathBuf;

pub fn load_skill_tool() -> ToolSpec {
    ToolSpec::object(
        "load_skill",
        "Load the full body of a named skill when specialized knowledge is needed.",
        json!({ "name": { "type": "string" } }),
        &["name"],
    )
}

pub fn skill_system_prompt(descriptions: &str) -> String {
    format!(
        "You are a coding agent. Use load_skill before unfamiliar specialized work.\nSkills available:\n{descriptions}"
    )
}

pub struct SkillAgent {
    client: OpenAiClient,
    tools: ToolRegistry,
    skills: SkillLoader,
}

impl SkillAgent {
    pub fn new(workspace: PathBuf) -> Result<Self> {
        Ok(Self {
            client: OpenAiClient::from_env()?,
            tools: ToolRegistry::new(workspace.clone()),
            skills: SkillLoader::from_dir(workspace.join("skills"))?,
        })
    }

    pub async fn run(&self, prompt: &str) -> Result<String> {
        let mut tool_specs = self.tools.specs();
        tool_specs.push(load_skill_tool());
        let mut messages = vec![
            ChatMessage::system(skill_system_prompt(&self.skills.descriptions())),
            ChatMessage::user(prompt),
        ];

        for _ in 0..30 {
            let response = self.client.chat(&messages, &tool_specs).await?;
            let calls = response.tool_calls.clone().unwrap_or_default();
            messages.push(response);

            if calls.is_empty() {
                return Ok(messages
                    .last()
                    .and_then(|message| message.content.clone())
                    .unwrap_or_default());
            }

            for call in calls {
                let args: Value = serde_json::from_str(&call.function.arguments)?;
                let output = if call.function.name == "load_skill" {
                    self.skills.load(args["name"].as_str().unwrap_or_default())?
                } else {
                    self.tools.execute(&call.function.name, args)?
                };
                messages.push(ChatMessage::tool(call.id, output));
            }
        }

        Ok("stopped after safety round limit".to_string())
    }
}
