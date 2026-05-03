fn main() -> anyhow::Result<()> {
    use ai_agent_learning_openai::lessons::lesson03_todo_write::{TodoItem, TodoManager, TodoStatus};
    let mut todo = TodoManager::default();
    let rendered = todo.update(vec![TodoItem {
        content: "Port the agent tutorial to Rust".to_string(),
        status: TodoStatus::InProgress,
        active_form: Some("writing readable Rust".to_string()),
    }])?;
    println!("{rendered}");
    Ok(())
}

