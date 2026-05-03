use crate::openai::{ChatMessage, OpenAiClient, ToolSpec};
use anyhow::Result;
use serde_json::{json, Value};
use std::process::Command;

const SYSTEM: &str = "You are a coding agent in the current workspace. Use bash to solve tasks. Act, then explain briefly.";
const MAX_ROUNDS: usize = 30;

pub fn bash_tool() -> ToolSpec {
    ToolSpec::object(
        "bash",
        "Run a shell command in the current workspace.",
        json!({
            "command": {
                "type": "string",
                "description": "The shell command to run"
            }
        }),
        &["command"],
    )
}

pub fn run_bash(command: &str) -> String {
    let blocked = ["rm -rf /", "sudo ", "shutdown", "reboot", "> /dev/"];
    if blocked.iter().any(|needle| command.contains(needle)) {
        return "Error: dangerous command blocked".to_string();
    }

    let output = if cfg!(windows) {
        Command::new("powershell")
            .args(["-NoProfile", "-Command", command])
            .output()
    } else {
        Command::new("sh").arg("-c").arg(command).output()
    };

    match output {
        Ok(output) => {
            let mut text = String::new();
            text.push_str(&String::from_utf8_lossy(&output.stdout));
            text.push_str(&String::from_utf8_lossy(&output.stderr));
            let text = text.trim();
            if text.is_empty() {
                "(no output)".to_string()
            } else {
                text.chars().take(50_000).collect()
            }
        }
        Err(err) => format!("Error: {err}"),
    }
}

pub async fn agent_loop(client: &OpenAiClient, messages: &mut Vec<ChatMessage>) -> Result<()> {
    let tools = vec![bash_tool()];

    for _ in 0..MAX_ROUNDS {
        let response = client.chat(messages, &tools).await?;
        let tool_calls = response.tool_calls.clone().unwrap_or_default();
        messages.push(response);

        if tool_calls.is_empty() {
            return Ok(());
        }

        for call in tool_calls {
            let args: Value = serde_json::from_str(&call.function.arguments)?;
            let command = args["command"].as_str().unwrap_or_default();
            println!("$ {command}");
            let output = run_bash(command);
            println!("{}", output.chars().take(200).collect::<String>());
            messages.push(ChatMessage::tool(call.id, output));
        }
    }

    messages.push(ChatMessage::user(
        "Stopped because the safety round limit was reached.",
    ));
    Ok(())
}

pub async fn run(prompt: &str) -> Result<String> {
    let client = OpenAiClient::from_env()?;
    let mut messages = vec![ChatMessage::system(SYSTEM), ChatMessage::user(prompt)];
    agent_loop(&client, &mut messages).await?;
    Ok(messages
        .iter()
        .rev()
        .find_map(|message| message.content.clone())
        .unwrap_or_default())
}

