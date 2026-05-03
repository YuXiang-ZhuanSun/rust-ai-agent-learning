#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let prompt = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    println!("{}", ai_agent_learning_openai::lessons::lesson01_tool_use::run(&prompt).await?);
    Ok(())
}

