use crate::runtime::MessageBus;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolRequest {
    pub request_id: String,
    pub teammate: String,
    pub kind: String,
    pub status: String,
}

#[derive(Debug, Default, Clone)]
pub struct ProtocolTracker {
    requests: BTreeMap<String, ProtocolRequest>,
}

impl ProtocolTracker {
    pub fn record(&mut self, teammate: &str, kind: &str) -> String {
        let request_id = Uuid::new_v4().to_string();
        self.requests.insert(
            request_id.clone(),
            ProtocolRequest {
                request_id: request_id.clone(),
                teammate: teammate.to_string(),
                kind: kind.to_string(),
                status: "pending".to_string(),
            },
        );
        request_id
    }

    pub fn resolve(&mut self, request_id: &str, status: &str) -> Result<ProtocolRequest> {
        let request = self
            .requests
            .get_mut(request_id)
            .ok_or_else(|| anyhow::anyhow!("unknown protocol request: {request_id}"))?;
        request.status = status.to_string();
        Ok(request.clone())
    }

    pub fn list(&self) -> Vec<ProtocolRequest> {
        self.requests.values().cloned().collect()
    }
}

pub fn request_shutdown(bus: &MessageBus, from: &str, to: &str, reason: &str) -> Result<String> {
    let request_id = Uuid::new_v4().to_string();
    bus.send_with_request(
        from,
        to,
        "shutdown_request",
        json!({ "reason": reason }),
        Some(request_id.clone()),
    )?;
    Ok(request_id)
}

pub fn approve_plan(
    bus: &MessageBus,
    from: &str,
    to: &str,
    request_id: &str,
    approved: bool,
) -> Result<()> {
    bus.send_with_request(
        from,
        to,
        "plan_approval",
        json!({ "approved": approved }),
        Some(request_id.to_string()),
    )?;
    Ok(())
}

pub struct TeamProtocols {
    bus: MessageBus,
    tracker: ProtocolTracker,
}

impl TeamProtocols {
    pub fn new(bus: MessageBus) -> Self {
        Self {
            bus,
            tracker: ProtocolTracker::default(),
        }
    }

    pub fn shutdown_request(&mut self, teammate: &str, reason: &str) -> Result<String> {
        let request_id = self.tracker.record(teammate, "shutdown_request");
        self.bus.send_with_request(
            "lead",
            teammate,
            "shutdown_request",
            json!({ "reason": reason }),
            Some(request_id.clone()),
        )?;
        Ok(request_id)
    }

    pub fn plan_decision(
        &mut self,
        teammate: &str,
        request_id: &str,
        approved: bool,
        feedback: &str,
    ) -> Result<()> {
        self.bus.send_with_request(
            "lead",
            teammate,
            "plan_approval",
            json!({ "approved": approved, "feedback": feedback }),
            Some(request_id.to_string()),
        )?;
        let status = if approved { "approved" } else { "rejected" };
        self.tracker.resolve(request_id, status)?;
        Ok(())
    }

    pub fn pending(&self) -> Vec<ProtocolRequest> {
        self.tracker
            .list()
            .into_iter()
            .filter(|request| request.status == "pending")
            .collect()
    }
}
