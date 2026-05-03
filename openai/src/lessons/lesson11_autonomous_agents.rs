use crate::runtime::{Task, TaskManager, TaskStatus};
use anyhow::Result;
use serde::{Deserialize, Serialize};

pub fn identity_block(name: &str, role: &str) -> String {
    format!("<identity>\nname: {name}\nrole: {role}\n</identity>")
}

pub fn claim_next_ready_task(tasks: &TaskManager, owner: &str) -> Result<Option<Task>> {
    for task in tasks.list()? {
        if task.status == TaskStatus::Pending && task.blocked_by.is_empty() && task.owner.is_none()
        {
            return Ok(Some(tasks.claim(task.id, owner)?));
        }
    }
    Ok(None)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdleDecision {
    ResumeFromInbox(String),
    ClaimTask(Task),
    StayIdle,
}

pub struct AutonomousPolicy {
    pub name: String,
    pub role: String,
}

impl AutonomousPolicy {
    pub fn new(name: &str, role: &str) -> Self {
        Self {
            name: name.to_string(),
            role: role.to_string(),
        }
    }

    pub fn identity_message(&self) -> String {
        identity_block(&self.name, &self.role)
    }

    pub fn decide(
        &self,
        tasks: &TaskManager,
        inbox_messages: Vec<String>,
    ) -> Result<IdleDecision> {
        if let Some(message) = inbox_messages.into_iter().next() {
            return Ok(IdleDecision::ResumeFromInbox(message));
        }

        if let Some(task) = claim_next_ready_task(tasks, &self.name)? {
            return Ok(IdleDecision::ClaimTask(task));
        }

        Ok(IdleDecision::StayIdle)
    }
}
