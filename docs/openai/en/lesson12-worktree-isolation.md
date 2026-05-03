# Lesson 12: Worktree Isolation

```text
L00 > L01 > L02 > L03 > L04 > L05 > L06 > L07 > L08 > L09 > L10 > L11 > [ L12 ] > L13
```

> 按任务协调，按目录隔离。

## Problem

多个 Agent 同时改一个工作目录，冲突会非常难排查。工作树隔离把任务和执行目录绑定：任务仍在 `.tasks` 里协调，具体修改发生在 `.worktrees/<name>`。这样并行工作既能共享任务板，又不会踩同一份文件。

## Solution

```text
Task #12 -> worktree auth-refactor -> .worktrees/auth-refactor
Task #13 -> worktree ui-copy       -> .worktrees/ui-copy
EventBus records create/run/remove
```

## How It Works

1. 为任务创建独立 worktree lane。
2. 把 task_id、owner、path 记录到索引或任务字段。
3. Agent 在自己的目录里运行命令和编辑文件。
4. EventBus 记录 create/remove/run 等生命周期事件。
5. 任务完成后合并或清理 worktree。

## Mechanism

| Component | Role |
|-----------|------|
| 控制平面 | 任务板管理目标和状态 |
| 执行平面 | worktree 管理目录隔离 |
| 事件流 | 生命周期可观察 |
| 并行 | 不同 Agent 不直接抢同一目录 |

## Source Slice

```rust
let events = EventBus::default();
let worktrees = WorktreeManager::new(".worktrees", events.clone())?;
let path = worktrees.create("auth-refactor")?;
```

Full source: `openai/src/lessons/lesson12_worktree_isolation.rs`. Entry point: `openai/src/bin/lesson12.rs`.

## Try It

```bash
cargo run -p ai-agent-learning-openai --bin lesson12 -- "为两个任务创建两个 worktree"
```

More prompts:

- 查看 worktree_events
- 删除已完成任务的 worktree
