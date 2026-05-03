use crate::runtime::{MessageBus, TaskManager};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Teammate {
    pub name: String,
    pub role: String,
    pub prompt: String,
}

#[derive(Debug, Default, Clone)]
pub struct TeammateManager {
    teammates: BTreeMap<String, Teammate>,
}

impl TeammateManager {
    pub fn spawn(&mut self, name: &str, role: &str, prompt: &str) -> String {
        self.teammates.insert(
            name.to_string(),
            Teammate {
                name: name.to_string(),
                role: role.to_string(),
                prompt: prompt.to_string(),
            },
        );
        format!("spawned teammate {name} as {role}")
    }

    pub fn names(&self) -> Vec<String> {
        self.teammates.keys().cloned().collect()
    }
}

pub fn delegate(bus: &MessageBus, from: &str, to: &str, task: &str) -> Result<String> {
    let msg = bus.send(from, to, "task", json!({ "task": task }))?;
    Ok(msg.id)
}

pub fn board_snapshot(tasks: &TaskManager) -> Result<String> {
    Ok(serde_json::to_string_pretty(&tasks.list()?)?)
}

