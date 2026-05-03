use crate::openai::ToolSpec;
pub use crate::runtime::{Task, TaskManager, TaskStatus};
use serde_json::json;

pub fn task_tools() -> Vec<ToolSpec> {
    vec![
        ToolSpec::object(
            "task_create",
            "Create a persistent task on the task board.",
            json!({
                "subject": { "type": "string" },
                "description": { "type": "string" }
            }),
            &["subject"],
        ),
        ToolSpec::object(
            "task_update",
            "Update task status and dependency edges.",
            json!({
                "id": { "type": "integer" },
                "status": {
                    "type": "string",
                    "enum": ["pending", "in_progress", "blocked", "completed"]
                },
                "blocked_by": { "type": "array", "items": { "type": "integer" } },
                "blocks": { "type": "array", "items": { "type": "integer" } }
            }),
            &["id"],
        ),
        ToolSpec::object(
            "task_list",
            "List all tasks with status and dependency summary.",
            json!({}),
            &[],
        ),
        ToolSpec::object(
            "task_get",
            "Read full details for one task.",
            json!({ "id": { "type": "integer" } }),
            &["id"],
        ),
    ]
}

pub fn explain_task_graph() -> &'static str {
    "Tasks are durable goals. Dependencies tell the agent what is ready, what is blocked, and what can run in parallel."
}

