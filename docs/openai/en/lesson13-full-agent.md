# Lesson 13: Full Reference Agent

```text
L00 > L01 > L02 > L03 > L04 > L05 > L06 > L07 > L08 > L09 > L10 > L11 > L12 > [ L13 ]
```

> 完整 Agent 不是一个大提示词，而是一组状态机制围绕同一个循环协作。

## Problem

真实 Agent 需要同时处理很多东西：工具、计划、技能、压缩、后台任务、队友消息、协议状态。完整参考 Agent 的价值不是“代码更长”，而是展示这些机制在同一个循环里如何排序：哪些在 LLM 调用前注入，哪些在工具执行后更新，哪些要持久化。

## Solution

```text
before LLM: micro_compact -> drain background -> read inbox
LLM call: all tool schemas
after tool calls: dispatch -> update managers -> maybe compact
state: todo / skills / tasks / bg / bus / team
```

## How It Works

1. 初始化所有 manager：Todo、Skill、Task、Background、MessageBus、Team。
2. 每轮先做上下文维护和外部通知注入。
3. 一次性注册所有工具 schema。
4. 按工具名分发到对应 manager。
5. 根据工具结果更新消息、状态和持久化文件。

## Mechanism

| Component | Role |
|-----------|------|
| 组合 | 机制按顺序进入同一个循环 |
| 状态 | 每类状态有自己的 manager |
| 可恢复 | 任务、消息、transcript 都在文件系统中 |
| 参考价值 | 可作为真实 Agent 的最小架构草图 |

## Source Slice

```rust
micro_compact(&mut messages, 8, 500);
for note in self.background.drain() { inject(note); }
let inbox = self.bus.read_inbox("lead")?;
let reply = self.client.chat(&messages, &self.tool_specs()).await?;
let output = self.execute_tool(&call.function.name, args)?;
```

Full source: `openai/src/lessons/lesson13_full_agent.rs`. Entry point: `openai/src/bin/lesson13.rs`.

## Try It

```bash
cargo run -p ai-agent-learning-openai --bin lesson13 -- "让完整 Agent 创建任务板并规划项目"
```

More prompts:

- 启动后台测试并继续编辑文件
- 生成队友并通过 inbox 发送任务
