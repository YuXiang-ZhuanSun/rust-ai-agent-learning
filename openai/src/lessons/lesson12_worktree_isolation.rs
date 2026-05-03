use crate::openai::ToolSpec;
pub use crate::runtime::{EventBus, WorktreeManager};
use serde_json::json;

pub fn worktree_tools() -> Vec<ToolSpec> {
    vec![
        ToolSpec::object(
            "worktree_create",
            "Create an isolated directory lane for a task.",
            json!({
                "name": { "type": "string" },
                "task_id": { "type": "integer" },
                "owner": { "type": "string" }
            }),
            &["name"],
        ),
        ToolSpec::object(
            "worktree_remove",
            "Remove an isolated worktree directory after the task is finished.",
            json!({ "name": { "type": "string" } }),
            &["name"],
        ),
        ToolSpec::object(
            "worktree_events",
            "Read lifecycle events for worktree creation, use, and cleanup.",
            json!({}),
            &[],
        ),
    ]
}

pub fn core_insight() -> &'static str {
    "Tasks coordinate intent; worktrees isolate execution. Tie them together with task_id."
}

