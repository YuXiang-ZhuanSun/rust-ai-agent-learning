use crate::openai::ToolSpec;
pub use crate::runtime::{BackgroundManager, BackgroundTask};
use serde_json::json;

pub fn background_tools() -> Vec<ToolSpec> {
    vec![
        ToolSpec::object(
            "background_run",
            "Run a slow command in a background thread and return a task id immediately.",
            json!({
                "command": { "type": "string" },
                "timeout_secs": { "type": "integer" }
            }),
            &["command"],
        ),
        ToolSpec::object(
            "check_background",
            "Check one background task, or let the agent inspect completed notifications.",
            json!({ "id": { "type": "string" } }),
            &[],
        ),
    ]
}

pub fn render_notifications(tasks: Vec<BackgroundTask>) -> String {
    if tasks.is_empty() {
        return String::new();
    }
    let mut out = String::from("<background-results>\n");
    for task in tasks {
        out.push_str(&format!(
            "[bg:{}] {}: {}\n",
            task.id,
            task.status,
            task.result.unwrap_or_default()
        ));
    }
    out.push_str("</background-results>");
    out
}

