# Lesson 12: 工作树隔离

```text
L00 > L01 > L02 > L03 > L04 > L05 > L06 > L07 > L08 > L09 > L10 > L11 > [ L12 ] > L13
```

> 按任务协调，按目录隔离。

## 问题

多个 Agent 同时改一个工作目录，冲突会非常难排查。工作树隔离把任务和执行目录绑定：任务仍在 `.tasks` 里协调，具体修改发生在 `.worktrees/<name>`。这样并行工作既能共享任务板，又不会踩同一份文件。

## 解决方案

```text
Task #12 -> worktree auth-refactor -> .worktrees/auth-refactor
Task #13 -> worktree ui-copy       -> .worktrees/ui-copy
EventBus records create/run/remove
```

## 工作原理

1. 为任务创建独立 worktree lane。
2. 把 task_id、owner、path 记录到索引或任务字段。
3. Agent 在自己的目录里运行命令和编辑文件。
4. EventBus 记录 create/remove/run 等生命周期事件。
5. 任务完成后合并或清理 worktree。

## 本章机制

| 组件 | 作用 |
|------|------|
| 控制平面 | 任务板管理目标和状态 |
| 执行平面 | worktree 管理目录隔离 |
| 事件流 | 生命周期可观察 |
| 并行 | 不同 Agent 不直接抢同一目录 |

## 源码切片

```rust
let events = EventBus::default();
let worktrees = WorktreeManager::new(".worktrees", events.clone())?;
let path = worktrees.create("auth-refactor")?;
```

完整源码：`openai/src/lessons/lesson12_worktree_isolation.rs`。运行入口：`openai/src/bin/lesson12.rs`。

## 试一试

```bash
cargo run -p ai-agent-learning-openai --bin lesson12 -- "为两个任务创建两个 worktree"
```

还可以试：

- 查看 worktree_events
- 删除已完成任务的 worktree
