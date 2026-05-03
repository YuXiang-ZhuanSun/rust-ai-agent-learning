# Lesson 8: 后台任务

```text
L00 > L01 > L02 > L03 > L04 > L05 > L06 > L07 > [ L08 ] > L09 > L10 > L11 > L12 > L13
```

> 慢任务不该冻结 Agent 的思考。

## 问题

构建、测试、搜索、下载都可能很慢。如果主循环同步等待，Agent 就什么也做不了。后台任务机制把慢命令放到线程里执行，立即返回 task id；主循环之后每轮检查完成通知，把结果注入对话。

## 解决方案

```text
main loop: background_run -> task_id -> continue
background thread: command -> result -> notification queue
next turn: drain queue -> <background-results>
```

## 工作原理

1. `background_run` 创建后台任务并立即返回 id。
2. 线程执行命令并记录状态、输出、错误。
3. 完成后把通知放入队列。
4. 主循环每轮 LLM 调用前 drain 队列。
5. 模型拿到结果后决定下一步。

## 本章机制

| 组件 | 作用 |
|------|------|
| 非阻塞 | 主循环不等待慢命令 |
| 通知 | 结果以 `<background-results>` 形式回填 |
| 查询 | check_background 可查看任务状态 |
| 适用 | 测试、构建、长搜索 |

## 源码切片

```rust
let id = background.run("cargo test", 120);
for note in background.drain() {
    messages.push(ChatMessage::user(render_background(note)));
}
```

完整源码：`openai/src/lessons/lesson08_background_tasks.rs`。运行入口：`openai/src/bin/lesson08.rs`。

## 试一试

```bash
cargo run -p ai-agent-learning-openai --bin lesson08 -- "后台运行 cargo test"
```

还可以试：

- 同时让 Agent 继续读文件
- 完成后解释测试输出
