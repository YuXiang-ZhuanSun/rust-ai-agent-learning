fn main() -> anyhow::Result<()> {
    let bus = ai_agent_learning_openai::MessageBus::new(".team/inbox")?;
    let request_id = ai_agent_learning_openai::lessons::lesson10_team_protocols::request_shutdown(
        &bus,
        "lead",
        "alice",
        "demo complete",
    )?;
    println!("shutdown request {request_id}");
    Ok(())
}

