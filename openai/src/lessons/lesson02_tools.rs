use crate::openai::ToolSpec;
use crate::runtime::run_shell;
use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

pub type ToolHandler = fn(&Path, Value) -> Result<String>;

#[derive(Clone)]
pub struct ToolRegistry {
    root: PathBuf,
    handlers: BTreeMap<String, ToolHandler>,
}

impl ToolRegistry {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        let mut registry = Self {
            root: root.into(),
            handlers: BTreeMap::new(),
        };
        registry.add("bash", bash);
        registry.add("read_file", read_file);
        registry.add("write_file", write_file);
        registry.add("edit_file", edit_file);
        registry
    }

    pub fn add(&mut self, name: &str, handler: ToolHandler) {
        self.handlers.insert(name.to_string(), handler);
    }

    pub fn execute(&self, name: &str, args: Value) -> Result<String> {
        let handler = self
            .handlers
            .get(name)
            .ok_or_else(|| anyhow!("unknown tool: {name}"))?;
        handler(&self.root, args)
    }

    pub fn specs(&self) -> Vec<ToolSpec> {
        vec![
            ToolSpec::object(
                "bash",
                "Run a shell command in the workspace.",
                json!({ "command": { "type": "string" }, "timeout_secs": { "type": "integer" } }),
                &["command"],
            ),
            ToolSpec::object(
                "read_file",
                "Read a UTF-8 file from the workspace.",
                json!({ "path": { "type": "string" } }),
                &["path"],
            ),
            ToolSpec::object(
                "write_file",
                "Write a UTF-8 file inside the workspace.",
                json!({ "path": { "type": "string" }, "content": { "type": "string" } }),
                &["path", "content"],
            ),
            ToolSpec::object(
                "edit_file",
                "Replace an exact string in a UTF-8 file.",
                json!({
                    "path": { "type": "string" },
                    "old": { "type": "string" },
                    "new": { "type": "string" }
                }),
                &["path", "old", "new"],
            ),
        ]
    }
}

fn workspace_path(root: &Path, raw: &str) -> Result<PathBuf> {
    let requested = Path::new(raw);
    if requested.is_absolute()
        || requested
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(anyhow!("path escapes workspace: {raw}"));
    }

    let joined = root.join(requested);
    let parent = joined.parent().unwrap_or(root);
    fs::create_dir_all(parent)?;
    Ok(joined)
}

fn bash(_root: &Path, args: Value) -> Result<String> {
    let command = args["command"].as_str().context("missing command")?;
    let timeout = args["timeout_secs"].as_u64().unwrap_or(30);
    Ok(run_shell(command, timeout))
}

fn read_file(root: &Path, args: Value) -> Result<String> {
    let path = workspace_path(root, args["path"].as_str().context("missing path")?)?;
    fs::read_to_string(&path).with_context(|| format!("reading {}", path.display()))
}

fn write_file(root: &Path, args: Value) -> Result<String> {
    let path = workspace_path(root, args["path"].as_str().context("missing path")?)?;
    fs::write(&path, args["content"].as_str().unwrap_or_default())?;
    Ok(format!("wrote {}", path.display()))
}

fn edit_file(root: &Path, args: Value) -> Result<String> {
    let path = workspace_path(root, args["path"].as_str().context("missing path")?)?;
    let old = args["old"].as_str().context("missing old")?;
    let new = args["new"].as_str().context("missing new")?;
    let text = fs::read_to_string(&path)?;
    if !text.contains(old) {
        return Err(anyhow!("old text not found"));
    }
    fs::write(&path, text.replacen(old, new, 1))?;
    Ok(format!("edited {}", path.display()))
}
