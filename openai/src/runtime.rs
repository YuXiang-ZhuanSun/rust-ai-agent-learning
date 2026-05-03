use anyhow::{anyhow, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TodoStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TodoItem {
    pub content: String,
    pub status: TodoStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_form: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct TodoManager {
    items: Vec<TodoItem>,
}

impl TodoManager {
    pub fn update(&mut self, items: Vec<TodoItem>) -> Result<String> {
        if items.len() > 20 {
            return Err(anyhow!("todo list may contain at most 20 items"));
        }
        let active = items
            .iter()
            .filter(|item| item.status == TodoStatus::InProgress)
            .count();
        if active > 1 {
            return Err(anyhow!("only one todo may be in_progress"));
        }
        self.items = items;
        Ok(self.render())
    }

    pub fn render(&self) -> String {
        if self.items.is_empty() {
            return "No todos.".to_string();
        }

        let mut out = String::new();
        for item in &self.items {
            let mark = match item.status {
                TodoStatus::Pending => "[ ]",
                TodoStatus::InProgress => "[>]",
                TodoStatus::Completed => "[x]",
            };
            out.push_str(mark);
            out.push(' ');
            out.push_str(&item.content);
            if item.status == TodoStatus::InProgress {
                if let Some(active) = &item.active_form {
                    out.push_str(" <- ");
                    out.push_str(active);
                }
            }
            out.push('\n');
        }
        let done = self
            .items
            .iter()
            .filter(|item| item.status == TodoStatus::Completed)
            .count();
        out.push_str(&format!("\n({done}/{} completed)", self.items.len()));
        out
    }

    pub fn has_open_items(&self) -> bool {
        self.items
            .iter()
            .any(|item| item.status != TodoStatus::Completed)
    }
}

#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub body: String,
}

#[derive(Debug, Default, Clone)]
pub struct SkillLoader {
    skills: BTreeMap<String, Skill>,
}

impl SkillLoader {
    pub fn from_dir(root: impl AsRef<Path>) -> Result<Self> {
        let root = root.as_ref();
        let mut loader = Self::default();
        if !root.exists() {
            return Ok(loader);
        }
        loader.walk(root)?;
        Ok(loader)
    }

    fn walk(&mut self, dir: &Path) -> Result<()> {
        for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                self.walk(&path)?;
            } else if path.file_name().and_then(|s| s.to_str()) == Some("SKILL.md") {
                let body = fs::read_to_string(&path)
                    .with_context(|| format!("reading skill {}", path.display()))?;
                let name = path
                    .parent()
                    .and_then(Path::file_name)
                    .and_then(|s| s.to_str())
                    .unwrap_or("skill")
                    .to_string();
                let description = body
                    .lines()
                    .find(|line| line.starts_with("description:"))
                    .map(|line| line.trim_start_matches("description:").trim().to_string())
                    .unwrap_or_else(|| format!("Skill: {name}"));
                self.skills.insert(
                    name.clone(),
                    Skill {
                        name,
                        description,
                        body,
                    },
                );
            }
        }
        Ok(())
    }

    pub fn descriptions(&self) -> String {
        if self.skills.is_empty() {
            return "  (no skills found)".to_string();
        }
        self.skills
            .values()
            .map(|skill| format!("  - {}: {}", skill.name, skill.description))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn load(&self, name: &str) -> Result<String> {
        let skill = self
            .skills
            .get(name)
            .ok_or_else(|| anyhow!("unknown skill '{name}'"))?;
        Ok(format!(
            "<skill name=\"{}\">\n{}\n</skill>",
            skill.name, skill.body
        ))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Blocked,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: u64,
    pub subject: String,
    pub description: String,
    pub status: TaskStatus,
    #[serde(default)]
    pub blocked_by: Vec<u64>,
    #[serde(default)]
    pub blocks: Vec<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct TaskManager {
    root: PathBuf,
}

impl TaskManager {
    pub fn new(root: impl Into<PathBuf>) -> Result<Self> {
        let root = root.into();
        fs::create_dir_all(&root).with_context(|| format!("creating {}", root.display()))?;
        Ok(Self { root })
    }

    pub fn create(&self, subject: &str, description: &str) -> Result<Task> {
        let id = self.next_id()?;
        let now = Utc::now();
        let task = Task {
            id,
            subject: subject.to_string(),
            description: description.to_string(),
            status: TaskStatus::Pending,
            blocked_by: Vec::new(),
            blocks: Vec::new(),
            owner: None,
            created_at: now,
            updated_at: now,
        };
        self.save(&task)?;
        Ok(task)
    }

    pub fn get(&self, id: u64) -> Result<Task> {
        let path = self.path(id);
        let text = fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))?;
        serde_json::from_str(&text).with_context(|| format!("parsing {}", path.display()))
    }

    pub fn list(&self) -> Result<Vec<Task>> {
        let mut tasks = Vec::new();
        for entry in fs::read_dir(&self.root)? {
            let path = entry?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                let text = fs::read_to_string(&path)?;
                tasks.push(serde_json::from_str::<Task>(&text)?);
            }
        }
        tasks.sort_by_key(|task| task.id);
        Ok(tasks)
    }

    pub fn update(
        &self,
        id: u64,
        status: Option<TaskStatus>,
        blocked_by: Vec<u64>,
        blocks: Vec<u64>,
    ) -> Result<Task> {
        let mut task = self.get(id)?;
        if let Some(status) = status {
            task.status = status;
        }
        extend_unique(&mut task.blocked_by, blocked_by);
        extend_unique(&mut task.blocks, blocks);
        task.updated_at = Utc::now();
        self.save(&task)?;
        Ok(task)
    }

    pub fn claim(&self, id: u64, owner: &str) -> Result<Task> {
        let mut task = self.get(id)?;
        if task.owner.as_deref().is_some_and(|existing| existing != owner) {
            return Err(anyhow!("task {id} is already claimed"));
        }
        task.owner = Some(owner.to_string());
        task.status = TaskStatus::InProgress;
        task.updated_at = Utc::now();
        self.save(&task)?;
        Ok(task)
    }

    fn next_id(&self) -> Result<u64> {
        Ok(self.list()?.last().map(|task| task.id + 1).unwrap_or(1))
    }

    fn save(&self, task: &Task) -> Result<()> {
        let path = self.path(task.id);
        let text = serde_json::to_string_pretty(task)?;
        fs::write(&path, text).with_context(|| format!("writing {}", path.display()))
    }

    fn path(&self, id: u64) -> PathBuf {
        self.root.join(format!("task_{id}.json"))
    }
}

fn extend_unique(values: &mut Vec<u64>, additions: Vec<u64>) {
    for value in additions {
        if !values.contains(&value) {
            values.push(value);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackgroundTask {
    pub id: String,
    pub command: String,
    pub status: String,
    pub result: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct BackgroundManager {
    tasks: Arc<Mutex<HashMap<String, BackgroundTask>>>,
    notifications: Arc<Mutex<VecDeque<BackgroundTask>>>,
}

impl BackgroundManager {
    pub fn run(&self, command: &str, timeout_secs: u64) -> String {
        let id = Uuid::new_v4().to_string();
        let task = BackgroundTask {
            id: id.clone(),
            command: command.to_string(),
            status: "running".to_string(),
            result: None,
        };
        self.tasks.lock().unwrap().insert(id.clone(), task);

        let tasks = Arc::clone(&self.tasks);
        let notifications = Arc::clone(&self.notifications);
        let command = command.to_string();
        let id_for_thread = id.clone();
        thread::spawn(move || {
            let output = run_shell(&command, timeout_secs);
            let mut task = BackgroundTask {
                id: id_for_thread.clone(),
                command,
                status: "completed".to_string(),
                result: Some(output),
            };
            if task.result.as_deref().is_some_and(|text| text.starts_with("error:")) {
                task.status = "failed".to_string();
            }
            tasks.lock().unwrap().insert(id_for_thread, task.clone());
            notifications.lock().unwrap().push_back(task);
        });

        id
    }

    pub fn check(&self, id: &str) -> Option<BackgroundTask> {
        self.tasks.lock().unwrap().get(id).cloned()
    }

    pub fn drain(&self) -> Vec<BackgroundTask> {
        let mut queue = self.notifications.lock().unwrap();
        queue.drain(..).collect()
    }
}

pub fn run_shell(command: &str, timeout_secs: u64) -> String {
    let child = if cfg!(windows) {
        Command::new("powershell")
            .args(["-NoProfile", "-Command", command])
            .spawn()
    } else {
        Command::new("sh").arg("-c").arg(command).spawn()
    };

    let mut child = match child {
        Ok(child) => child,
        Err(err) => return format!("error: failed to spawn command: {err}"),
    };

    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(_)) => break,
            Ok(None) if start.elapsed() > Duration::from_secs(timeout_secs) => {
                let _ = child.kill();
                return "error: command timed out".to_string();
            }
            Ok(None) => thread::sleep(Duration::from_millis(25)),
            Err(err) => return format!("error: failed to wait for command: {err}"),
        }
    }

    match child.wait_with_output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            if output.status.success() {
                stdout.trim().to_string()
            } else {
                format!("error: {}\n{}", output.status, stderr.trim())
            }
        }
        Err(err) => format!("error: failed to collect output: {err}"),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub to: String,
    pub kind: String,
    pub body: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct MessageBus {
    inbox_dir: PathBuf,
}

impl MessageBus {
    pub fn new(inbox_dir: impl Into<PathBuf>) -> Result<Self> {
        let inbox_dir = inbox_dir.into();
        fs::create_dir_all(&inbox_dir)?;
        Ok(Self { inbox_dir })
    }

    pub fn send(&self, from: &str, to: &str, kind: &str, body: Value) -> Result<Message> {
        self.send_with_request(from, to, kind, body, None)
    }

    pub fn send_with_request(
        &self,
        from: &str,
        to: &str,
        kind: &str,
        body: Value,
        request_id: Option<String>,
    ) -> Result<Message> {
        let message = Message {
            id: Uuid::new_v4().to_string(),
            from: from.to_string(),
            to: to.to_string(),
            kind: kind.to_string(),
            body,
            request_id,
            created_at: Utc::now(),
        };
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(self.path(to))?;
        writeln!(file, "{}", serde_json::to_string(&message)?)?;
        Ok(message)
    }

    pub fn read_inbox(&self, name: &str) -> Result<Vec<Message>> {
        let path = self.path(name);
        if !path.exists() {
            return Ok(Vec::new());
        }
        let file = OpenOptions::new().read(true).open(&path)?;
        let reader = BufReader::new(file);
        let mut messages = Vec::new();
        for line in reader.lines() {
            let line = line?;
            if !line.trim().is_empty() {
                messages.push(serde_json::from_str(&line)?);
            }
        }
        fs::write(&path, "")?;
        Ok(messages)
    }

    pub fn broadcast(&self, from: &str, recipients: &[String], body: Value) -> Result<usize> {
        for recipient in recipients {
            self.send(from, recipient, "broadcast", body.clone())?;
        }
        Ok(recipients.len())
    }

    fn path(&self, name: &str) -> PathBuf {
        self.inbox_dir.join(format!("{name}.jsonl"))
    }
}

#[derive(Debug, Default, Clone)]
pub struct EventBus {
    events: Arc<Mutex<Vec<Value>>>,
}

impl EventBus {
    pub fn publish(&self, kind: &str, payload: Value) {
        self.events.lock().unwrap().push(json!({
            "kind": kind,
            "payload": payload,
            "created_at": Utc::now(),
        }));
    }

    pub fn drain(&self) -> Vec<Value> {
        self.events.lock().unwrap().drain(..).collect()
    }
}

#[derive(Debug, Clone)]
pub struct WorktreeManager {
    root: PathBuf,
    events: EventBus,
}

impl WorktreeManager {
    pub fn new(root: impl Into<PathBuf>, events: EventBus) -> Result<Self> {
        let root = root.into();
        fs::create_dir_all(&root)?;
        Ok(Self { root, events })
    }

    pub fn create(&self, name: &str) -> Result<PathBuf> {
        let clean = name
            .chars()
            .map(|ch| if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' { ch } else { '-' })
            .collect::<String>();
        let path = self.root.join(clean);
        fs::create_dir_all(&path)?;
        self.events
            .publish("worktree.created", json!({ "path": path.display().to_string() }));
        Ok(path)
    }

    pub fn remove(&self, name: &str) -> Result<()> {
        let path = self.root.join(name);
        if path.exists() {
            fs::remove_dir_all(&path)?;
        }
        self.events
            .publish("worktree.removed", json!({ "path": path.display().to_string() }));
        Ok(())
    }
}
