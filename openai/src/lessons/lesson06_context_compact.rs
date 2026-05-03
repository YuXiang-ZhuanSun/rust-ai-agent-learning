use crate::lessons::lesson02_tools::ToolRegistry;
use crate::openai::{ChatMessage, OpenAiClient, Role, ToolSpec};
use anyhow::Result;
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

pub fn estimate_tokens(messages: &[ChatMessage]) -> usize {
    messages
        .iter()
        .map(|message| message.text().split_whitespace().count() + 4)
        .sum()
}

pub fn micro_compact(messages: &mut [ChatMessage], keep_tail: usize, max_tool_chars: usize) {
    let compact_until = messages.len().saturating_sub(keep_tail);
    for message in messages.iter_mut().take(compact_until) {
        if message.role == Role::Tool {
            if let Some(content) = &mut message.content {
                if content.len() > max_tool_chars {
                    content.truncate(max_tool_chars);
                    content.push_str("\n...[truncated by micro compact]");
                }
            }
        }
    }
}

pub fn summary_replacement(summary: &str) -> Vec<ChatMessage> {
    vec![
        ChatMessage::system("Conversation was compacted. Continue from the summary."),
        ChatMessage::user(format!("<conversation-summary>\n{summary}\n</conversation-summary>")),
    ]
}

pub fn compress_tool() -> ToolSpec {
    ToolSpec::object(
        "compress",
        "Manually request context compaction after saving important state.",
        json!({ "reason": { "type": "string" } }),
        &[],
    )
}

pub struct CompactAgent {
    client: OpenAiClient,
    tools: ToolRegistry,
    transcript_dir: PathBuf,
    threshold: usize,
}

impl CompactAgent {
    pub fn new(workspace: PathBuf) -> Result<Self> {
        let transcript_dir = workspace.join(".transcripts");
        fs::create_dir_all(&transcript_dir)?;
        Ok(Self {
            client: OpenAiClient::from_env()?,
            tools: ToolRegistry::new(workspace),
            transcript_dir,
            threshold: 100_000,
        })
    }

    pub async fn run(&self, prompt: &str) -> Result<String> {
        let mut tool_specs = self.tools.specs();
        tool_specs.push(compress_tool());
        let mut messages = vec![
            ChatMessage::system("You are a coding agent. Use tools, and call compress when the conversation gets too large."),
            ChatMessage::user(prompt),
        ];

        for round in 0..30 {
            micro_compact(&mut messages, 8, 500);
            if estimate_tokens(&messages) > self.threshold {
                self.save_transcript(round, &messages)?;
                messages = summary_replacement("Conversation compacted. Preserve current goal, completed work, open tasks, and important file paths.");
            }

            let response = self.client.chat(&messages, &tool_specs).await?;
            let calls = response.tool_calls.clone().unwrap_or_default();
            messages.push(response);
            if calls.is_empty() {
                return Ok(messages.last().and_then(|m| m.content.clone()).unwrap_or_default());
            }

            let mut manual_compact = false;
            for call in calls {
                let args: Value = serde_json::from_str(&call.function.arguments)?;
                let output = if call.function.name == "compress" {
                    manual_compact = true;
                    "compression scheduled".to_string()
                } else {
                    self.tools.execute(&call.function.name, args)?
                };
                messages.push(ChatMessage::tool(call.id, output));
            }

            if manual_compact {
                self.save_transcript(round, &messages)?;
                messages = summary_replacement("Manual compaction requested. Continue from the preserved summary.");
            }
        }

        Ok("stopped after safety round limit".to_string())
    }

    fn save_transcript(&self, round: usize, messages: &[ChatMessage]) -> Result<()> {
        let path = self.transcript_dir.join(format!("round_{round}.json"));
        fs::write(path, serde_json::to_string_pretty(messages)?)?;
        Ok(())
    }
}
