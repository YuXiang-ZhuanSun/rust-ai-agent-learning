# Lesson 7: 任务系统

```text
L00 > L01 > L02 > L03 > L04 > L05 > L06 > [ L07 ] > L08 > L09 > L10 > L11 > L12 > L13
```

> Todo 管当前回合，Task 管跨会话目标。

## 问题

Todo 是短期计划，适合一个对话回合内推进；但真实项目常常跨越多轮会话、多个 Agent、多个依赖。任务系统把目标持久化为文件，并用依赖图表达“什么能做、什么被阻塞、完成哪个会释放下游”。

## 解决方案

```text
.tasks/task_1.json  completed
        | releases
.tasks/task_2.json  pending blocked_by=[1]
        |
        v
task_list shows ready / blocked work
```

## 工作原理

1. `task_create` 创建持久任务 JSON。
2. `task_update` 修改状态和依赖边。
3. `task_list` 给 Agent 一个任务板视图。
4. 任务完成时可释放被它阻塞的任务。
5. 多 Agent 可以通过同一个文件任务板协调。

## 本章机制

| 组件 | 作用 |
|------|------|
| 持久化 | 任务存到 `.tasks/task_N.json` |
| 依赖 | blocked_by / blocks 表示图关系 |
| 可检查 | 人和工具都能直接读 JSON |
| 长期性 | 压缩上下文后任务仍存在 |

## 源码切片

```rust
let task = tasks.create("实现登录", "拆分 API、UI、测试")?;
tasks.update(task.id, Some(TaskStatus::Completed), vec![], vec![])?;
```

完整源码：`openai/src/lessons/lesson07_task_system.rs`。运行入口：`openai/src/bin/lesson07.rs`。

## 试一试

```bash
cargo run -p ai-agent-learning-openai --bin lesson07 -- "创建三个有依赖的任务"
```

还可以试：

- 完成上游任务后查看列表
- 让两个 Agent 认领不同任务
