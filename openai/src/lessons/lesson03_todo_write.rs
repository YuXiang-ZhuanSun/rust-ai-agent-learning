use crate::lessons::lesson02_tools::ToolRegistry;
use crate::openai::{ChatMessage, OpenAiClient, ToolSpec};
pub use crate::runtime::{TodoItem, TodoManager, TodoStatus};
use anyhow::Result;
use serde_json::{json, Value};
use std::path::PathBuf;

pub fn todo_tool() -> ToolSpec {
    ToolSpec::object(
        "todo",
        "Replace the agent's visible task list. Use this before and during multi-step work.",
        json!({
            "items": {
                "type": "array",
                "items": {
                    "type": "object",
                    "properties": {
                        "content": { "type": "string" },
                        "status": {
                            "type": "string",
                            "enum": ["pending", "in_progress", "completed"]
                        },
                        "active_form": { "type": "string" }
                    },
                    "required": ["content", "status"]
                }
            }
        }),
        &["items"],
    )
}

pub struct TodoAgent {
    client: OpenAiClient,
    tools: ToolRegistry,
    todo: TodoManager,
}

impl TodoAgent {
    pub fn new(workspace: PathBuf) -> Result<Self> {
        Ok(Self {
            client: OpenAiClient::from_env()?,
            tools: ToolRegistry::new(workspace),
            todo: TodoManager::default(),
        })
    }

    pub async fn run(&mut self, prompt: &str) -> Result<String> {
        let mut tool_specs = self.tools.specs();
        tool_specs.push(todo_tool());
        let mut messages = vec![
            ChatMessage::system(
                "You are a coding agent. Use todo to plan multi-step work. Keep exactly one item in_progress while acting.",
            ),
            ChatMessage::user(prompt),
        ];
        let mut rounds_since_todo = 0;

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

            let mut used_todo = false;
            for call in calls {
                let args: Value = serde_json::from_str(&call.function.arguments)?;
                let output = if call.function.name == "todo" {
                    used_todo = true;
                    let items = parse_todos(args["items"].clone())?;
                    self.todo.update(items)?
                } else {
                    self.tools.execute(&call.function.name, args)?
                };
                messages.push(ChatMessage::tool(call.id, output));
            }

            rounds_since_todo = if used_todo { 0 } else { rounds_since_todo + 1 };
            if self.todo.has_open_items() && rounds_since_todo >= 3 {
                messages.push(ChatMessage::user(
                    "<reminder>You still have open todos. Update todo before continuing.</reminder>",
                ));
                rounds_since_todo = 0;
            }
        }

        Ok("stopped after safety round limit".to_string())
    }
}

fn parse_todos(value: Value) -> Result<Vec<TodoItem>> {
    let raw = value.as_array().cloned().unwrap_or_default();
    raw.into_iter()
        .map(|item| {
            let status = match item["status"].as_str().unwrap_or("pending") {
                "completed" => TodoStatus::Completed,
                "in_progress" => TodoStatus::InProgress,
                _ => TodoStatus::Pending,
            };
            Ok(TodoItem {
                content: item["content"].as_str().unwrap_or_default().to_string(),
                status,
                active_form: item["active_form"].as_str().map(str::to_string),
            })
        })
        .collect()
}
