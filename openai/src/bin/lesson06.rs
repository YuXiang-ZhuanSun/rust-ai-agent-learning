fn main() {
    use ai_agent_learning_openai::lessons::lesson06_context_compact::estimate_tokens;
    use ai_agent_learning_openai::ChatMessage;
    let messages = vec![ChatMessage::user("a small context")];
    println!("estimated tokens: {}", estimate_tokens(&messages));
}

