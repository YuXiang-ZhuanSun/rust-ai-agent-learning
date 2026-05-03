fn main() -> anyhow::Result<()> {
    let events = ai_agent_learning_openai::EventBus::default();
    let worktrees = ai_agent_learning_openai::WorktreeManager::new(".worktrees", events.clone())?;
    let path = worktrees.create("demo")?;
    println!("created {}", path.display());
    Ok(())
}

