fn main() -> anyhow::Result<()> {
    let tasks = ai_agent_learning_openai::lessons::lesson07_task_system::TaskManager::new(".tasks")?;
    let task = tasks.create("Try the Rust task board", "Create and inspect one persisted task")?;
    println!("{}", serde_json::to_string_pretty(&task)?);
    Ok(())
}

