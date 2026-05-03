# Lesson 10: Team Protocols

```text
L00 > L01 > L02 > L03 > L04 > L05 > L06 > L07 > L08 > L09 > [ L10 ] > L11 > L12 > L13
```

> 协议就是带 request_id 的消息。

## Problem

普通消息适合聊天，不适合审批和生命周期管理。比如队友请求关机，lead 需要知道这个请求是否已处理；队友提交计划，lead 要批准或驳回。协议层给消息增加 request_id 和状态追踪，让协作从“文本聊天”升级为“可跟踪流程”。

## Solution

```text
teammate -> shutdown_request(request_id=abc) -> lead
lead -> plan_approval(request_id=abc, approved=true)
tracker[abc] = approved
```

## How It Works

1. 创建协议请求时生成 request_id。
2. MessageBus 发送带 request_id 的消息。
3. ProtocolTracker 记录 pending 状态。
4. lead 做出决定后发送响应消息。
5. tracker 更新为 approved / rejected / closed。

## Mechanism

| Component | Role |
|-----------|------|
| request_id | 关联请求和响应 |
| tracker | 记录未决协议状态 |
| 类型 | shutdown_request、plan_approval 等 |
| 收益 | 协作状态可检查，不靠记忆 |

## Source Slice

```rust
let request_id = tracker.record("alice", "shutdown_request");
bus.send_with_request("lead", "alice", "shutdown_request", body, Some(request_id))?;
tracker.resolve(&request_id, "approved")?;
```

Full source: `openai/src/lessons/lesson10_team_protocols.rs`. Entry point: `openai/src/bin/lesson10.rs`.

## Try It

```bash
cargo run -p ai-agent-learning-openai --bin lesson10 -- "发起一个 shutdown_request"
```

More prompts:

- 列出 pending 请求
- 批准一个计划并检查状态
