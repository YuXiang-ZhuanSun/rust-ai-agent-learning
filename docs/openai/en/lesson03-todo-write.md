# Lesson 3: TodoWrite

```text
L00 > L01 > L02 > [ L03 ] > L04 > L05 > L06 > L07 > L08 > L09 > L10 > L11 > L12 > L13
```

> 计划要成为运行时状态，而不是一句“我会先做 A 再做 B”。

## Problem

长任务里，模型很容易被某个子问题吸走注意力。TodoWrite 的目的不是给用户一个漂亮列表，而是让 Agent 拥有一个可见、可更新、可约束的计划状态。它能提醒模型：当前做到哪一步，哪些还没做，什么时候该收尾。

## Solution

```text
User goal
   |
   v
todo([{ pending }, { in_progress }])
   |
   v
act with tools
   |
   v
update todo -> continue
```

## How It Works

1. 模型先用 `todo` 工具写出短计划。
2. `TodoManager` 校验最多 20 项，且只能有一个 `in_progress`。
3. 每次工具调用后检查是否长时间没更新 todo。
4. 如果仍有未完成项目，系统可注入轻量 reminder。
5. 完成任务时，todo 成为最终汇报的骨架。

## Mechanism

| Component | Role |
|-----------|------|
| 约束 | 只允许一个 in_progress，防止“并行假象” |
| 提醒 | 多轮未更新时注入 reminder |
| 可见性 | 用户能看到 Agent 的计划状态 |
| 可靠性 | 减少长任务漂移 |

## Source Slice

```rust
let mut used_todo = false;
for call in calls {
    if call.function.name == "todo" {
        used_todo = true;
        self.todo.update(parse_todos(args)?)?;
    }
}
if self.todo.has_open_items() && rounds_since_todo >= 3 {
    messages.push(ChatMessage::user("<reminder>Update your todos.</reminder>"));
}
```

Full source: `openai/src/lessons/lesson03_todo_write.rs`. Entry point: `openai/src/bin/lesson03.rs`.

## Try It

```bash
cargo run -p ai-agent-learning-openai --bin lesson03 -- "让 Agent 分三步创建一个小项目"
```

More prompts:

- 故意给很多任务，观察校验
- 比较有 todo 和无 todo 的执行稳定性
