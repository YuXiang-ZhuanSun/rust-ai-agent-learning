fn main() -> anyhow::Result<()> {
    let root = std::env::current_dir()?;
    let registry = ai_agent_learning_openai::lessons::lesson02_tools::ToolRegistry::new(root);
    println!("registered {} workspace tools", registry.specs().len());
    Ok(())
}

