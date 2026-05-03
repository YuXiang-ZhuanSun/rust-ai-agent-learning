# Lesson 11: Autonomous Agents

```text
L00 > L01 > L02 > L03 > L04 > L05 > L06 > L07 > L08 > L09 > L10 > [ L11 ] > L12 > L13
```

> 自治不是玄学，是空闲时该做什么的策略。

## Problem

如果队友只能等待 lead 指令，它们只是远程函数。自治 Agent 在完成当前工作后进入 idle phase：先读 inbox，再扫描任务板，找到无人认领且未阻塞的任务就 claim，然后重新进入工作阶段。

## Solution

```text
WORK phase: use tools until no tool call
        |
        v
IDLE phase: read inbox -> scan tasks -> claim ready task
        | message/task found
        v
resume WORK with identity block
```

## How It Works

1. 工作阶段正常执行工具循环。
2. 模型停止调用工具后，不直接退出，而是进入 idle phase。
3. 先读取 inbox，看是否有人发来新任务。
4. 再扫描 `.tasks`，找 pending、无 owner、无 blocked_by 的任务。
5. 认领任务后注入 identity block 和任务 prompt，继续工作。

## Mechanism

| Component | Role |
|-----------|------|
| idle policy | 空闲时的确定性策略 |
| claim | 防止多个 Agent 抢同一任务 |
| identity | 压缩后重新注入名字和角色 |
| 自治边界 | 只认领 ready task，不绕过依赖 |

## Source Slice

```rust
match policy.decide(&tasks, inbox_messages)? {
    IdleDecision::ResumeFromInbox(msg) => messages.push(ChatMessage::user(msg)),
    IdleDecision::ClaimTask(task) => messages.push(ChatMessage::user(format!("<auto-claimed>{}</auto-claimed>", task.subject))),
    IdleDecision::StayIdle => {}
}
```

Full source: `openai/src/lessons/lesson11_autonomous_agents.rs`. Entry point: `openai/src/bin/lesson11.rs`.

## Try It

```bash
cargo run -p ai-agent-learning-openai --bin lesson11 -- "创建一个 pending task"
```

More prompts:

- 让 alice 空闲后自动认领
- 给 alice inbox 写消息看它优先处理
