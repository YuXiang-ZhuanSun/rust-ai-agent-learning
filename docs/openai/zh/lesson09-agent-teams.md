# Lesson 9: Agent 团队

```text
L00 > L01 > L02 > L03 > L04 > L05 > L06 > L07 > L08 > [ L09 ] > L10 > L11 > L12 > L13
```

> 团队协作首先需要身份和邮箱。

## 问题

多个 Agent 如果只是在同一进程里跑几个循环，很快就会混乱：谁负责什么？消息发给谁？历史在哪里？Agent Teams 引入命名队友和 JSONL inbox。每个队友有名字、角色、提示词，通信写入对应 inbox 文件，方便持久化和审计。

## 解决方案

```text
.team/config.json: alice, bob
.team/inbox/alice.jsonl <- messages
.team/inbox/lead.jsonl  <- replies
lead loop drains inbox before thinking
```

## 工作原理

1. `spawn_teammate` 记录队友身份和角色。
2. `send_message` 向指定队友 inbox 追加 JSONL。
3. `read_inbox` 读取并清空收件箱。
4. `broadcast` 把同一消息发给所有队友。
5. Lead 每轮先检查 inbox，再调用模型。

## 本章机制

| 组件 | 作用 |
|------|------|
| 身份 | name + role + prompt |
| 邮箱 | 每个队友一个 JSONL 文件 |
| 异步 | 发送后不必立即等待回复 |
| 可观察 | 通信记录是普通文件 |

## 源码切片

```rust
let bus = MessageBus::new(".team/inbox")?;
bus.send("lead", "alice", "task", json!({ "task": "review API" }))?;
let messages = bus.read_inbox("lead")?;
```

完整源码：`openai/src/lessons/lesson09_agent_teams.rs`。运行入口：`openai/src/bin/lesson09.rs`。

## 试一试

```bash
cargo run -p ai-agent-learning-openai --bin lesson09 -- "创建 alice 和 bob 两个队友"
```

还可以试：

- 给 alice 发送审查任务
- 查看 .team/inbox 文件内容
