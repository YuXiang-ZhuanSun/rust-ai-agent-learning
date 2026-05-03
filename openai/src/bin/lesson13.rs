#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let prompt = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let prompt = if prompt.is_empty() { "Help me plan this project." } else { &prompt };
    let mut agent = ai_agent_learning_openai::lessons::lesson13_full_agent::FullAgent::new(std::env::current_dir()?)?;
    println!("{}", agent.run(prompt).await?);
    Ok(())
}

