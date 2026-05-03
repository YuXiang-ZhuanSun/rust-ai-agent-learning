#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let prompt = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    let prompt = if prompt.is_empty() { "Hello, what can you do?" } else { &prompt };
    println!("{}", ai_agent_learning_openai::lessons::lesson00_basic_chat::run(prompt).await?);
    Ok(())
}

