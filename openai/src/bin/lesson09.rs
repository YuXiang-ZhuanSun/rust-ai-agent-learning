fn main() -> anyhow::Result<()> {
    let bus = ai_agent_learning_openai::MessageBus::new(".team/inbox")?;
    let id = ai_agent_learning_openai::lessons::lesson09_agent_teams::delegate(
        &bus,
        "lead",
        "alice",
        "Review the plan",
    )?;
    println!("sent message {id}");
    Ok(())
}

