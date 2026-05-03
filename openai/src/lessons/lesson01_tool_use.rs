use crate::lessons::lesson00_basic_chat::{agent_loop as bash_agent_loop, bash_tool};
use crate::openai::{ChatMessage, OpenAiClient, ToolSpec};
use anyhow::Result;
use serde_json::{json, Value};

pub fn weather_tool() -> ToolSpec {
    ToolSpec::object(
        "get_weather",
        "Return a tiny weather report for a city.",
        json!({
            "city": { "type": "string", "description": "City name" }
        }),
        &["city"],
    )
}

pub fn execute_weather(arguments: &str) -> String {
    let args: Value = serde_json::from_str(arguments).unwrap_or_else(|_| json!({}));
    let city = args["city"].as_str().unwrap_or("unknown city");
    format!("{city}: sunny, 24C. This is a teaching stub.")
}

pub async fn run(prompt: &str) -> Result<String> {
    let client = OpenAiClient::from_env()?;
    let mut messages = vec![ChatMessage::system("You are a small agent that can use one domain tool. Use get_weather when the user asks about weather; otherwise answer directly."), ChatMessage::user(prompt)];
    let reply = client.chat(&messages, &[weather_tool()]).await?;

    if let Some(calls) = &reply.tool_calls {
        messages.push(reply.clone());
        for call in calls {
            let output = execute_weather(&call.function.arguments);
            messages.push(ChatMessage::tool(&call.id, output));
        }
        let final_reply = client.chat(&messages, &[weather_tool()]).await?;
        Ok(final_reply.text().to_string())
    } else {
        Ok(reply.text().to_string())
    }
}

pub async fn run_bash_style_agent(prompt: &str) -> Result<String> {
    let client = OpenAiClient::from_env()?;
    let mut messages = vec![
        ChatMessage::system("You are a coding agent. You have one real-world tool: bash."),
        ChatMessage::user(prompt),
    ];
    let _ = bash_tool();
    bash_agent_loop(&client, &mut messages).await?;
    Ok(messages
        .iter()
        .rev()
        .find_map(|message| message.content.clone())
        .unwrap_or_default())
}
