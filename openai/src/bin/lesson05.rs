fn main() -> anyhow::Result<()> {
    let loader = ai_agent_learning_openai::lessons::lesson05_skills::SkillLoader::from_dir("skills")?;
    println!("{}", loader.descriptions());
    Ok(())
}

