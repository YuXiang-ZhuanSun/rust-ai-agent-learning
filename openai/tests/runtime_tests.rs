use ai_agent_learning_openai::lessons::lesson02_tools::ToolRegistry;
use ai_agent_learning_openai::lessons::lesson06_context_compact::{estimate_tokens, micro_compact};
use ai_agent_learning_openai::{
    BackgroundManager, ChatMessage, EventBus, MessageBus, SkillLoader, TaskManager, TodoItem,
    TodoManager, TodoStatus, WorktreeManager,
};
use serde_json::json;

#[test]
fn todo_manager_allows_only_one_active_item() {
    let mut todo = TodoManager::default();
    let err = todo
        .update(vec![
            TodoItem {
                content: "one".into(),
                status: TodoStatus::InProgress,
                active_form: None,
            },
            TodoItem {
                content: "two".into(),
                status: TodoStatus::InProgress,
                active_form: None,
            },
        ])
        .unwrap_err();
    assert!(err.to_string().contains("only one"));
}

#[test]
fn skill_loader_discovers_skill_files() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let skill_dir = dir.path().join("git");
    std::fs::create_dir_all(&skill_dir)?;
    std::fs::write(skill_dir.join("SKILL.md"), "description: Git workflow\nBody")?;

    let loader = SkillLoader::from_dir(dir.path())?;
    assert!(loader.descriptions().contains("Git workflow"));
    assert!(loader.load("git")?.contains("<skill name=\"git\">"));
    Ok(())
}

#[test]
fn task_manager_persists_and_claims_tasks() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let tasks = TaskManager::new(dir.path())?;
    let task = tasks.create("subject", "description")?;
    let claimed = tasks.claim(task.id, "alice")?;
    assert_eq!(claimed.owner.as_deref(), Some("alice"));
    assert_eq!(tasks.list()?.len(), 1);
    Ok(())
}

#[test]
fn message_bus_sends_reads_and_drains_jsonl() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let bus = MessageBus::new(dir.path())?;
    bus.send("lead", "alice", "task", json!({ "body": "hello" }))?;
    assert_eq!(bus.read_inbox("alice")?.len(), 1);
    assert!(bus.read_inbox("alice")?.is_empty());
    Ok(())
}

#[test]
fn tool_registry_reads_writes_and_edits_files() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let tools = ToolRegistry::new(dir.path());
    tools.execute("write_file", json!({ "path": "a.txt", "content": "hello" }))?;
    tools.execute("edit_file", json!({ "path": "a.txt", "old": "hello", "new": "hi" }))?;
    let text = tools.execute("read_file", json!({ "path": "a.txt" }))?;
    assert_eq!(text, "hi");
    Ok(())
}

#[test]
fn compact_truncates_old_tool_messages() {
    let mut messages = vec![
        ChatMessage::tool("call-1", "x".repeat(100)),
        ChatMessage::user("tail"),
    ];
    micro_compact(&mut messages, 1, 10);
    assert!(messages[0].text().contains("truncated"));
    assert!(estimate_tokens(&messages) > 0);
}

#[test]
fn background_manager_reports_completion() {
    let manager = BackgroundManager::default();
    let id = manager.run("echo ok", 5);
    std::thread::sleep(std::time::Duration::from_millis(500));
    let task = manager.check(&id).expect("task exists");
    assert!(["running", "completed"].contains(&task.status.as_str()));
}

#[test]
fn worktree_manager_emits_events() -> anyhow::Result<()> {
    let dir = tempfile::tempdir()?;
    let events = EventBus::default();
    let manager = WorktreeManager::new(dir.path(), events.clone())?;
    manager.create("feature/test")?;
    assert_eq!(events.drain().len(), 1);
    Ok(())
}

