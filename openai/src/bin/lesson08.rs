fn main() {
    let bg = ai_agent_learning_openai::lessons::lesson08_background_tasks::BackgroundManager::default();
    let id = bg.run("echo background-ok", 5);
    println!("started background task {id}");
}

