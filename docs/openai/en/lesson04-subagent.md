# Lesson 4: Subagents

```text
L00 > L01 > L02 > L03 > [ L04 ] > L05 > L06 > L07 > L08 > L09 > L10 > L11 > L12 > L13
```

> 委托的本质不是多开一个模型，而是隔离上下文。

## Problem

探索型子任务会制造大量噪音：读十几个文件、跑一堆命令、试错很多路径。父 Agent 通常只需要结论，不需要所有过程。子 Agent 用全新的消息列表完成任务，最后只把摘要作为 tool_result 交还给父 Agent，从而保持主上下文干净。

## Solution

```text
Parent messages=[main task]
        | task(prompt)
        v
Child messages=[] + base tools
        | explores files, runs commands
        v
summary only -> parent tool_result
```

## How It Works

1. 父 Agent 暴露 `task` 工具，参数是子任务 prompt 和简短描述。
2. 子 Agent 使用新 `messages`，不继承父对话历史。
3. 子 Agent 有基础工具，但没有 `task`，避免递归生成。
4. 子 Agent 自己运行工具循环，直到停止或达到轮数上限。
5. 父 Agent 只收到最终摘要，继续主任务。

## Mechanism

| Component | Role |
|-----------|------|
| 隔离 | 子任务噪音不会污染主上下文 |
| 共享 | 父子共享文件系统，所以能协作修改项目 |
| 限制 | 子 Agent 不再拥有 task 工具 |
| 返回 | 只返回摘要，不返回完整消息历史 |

## Source Slice

```rust
pub async fn run(&self, prompt: &str, agent_type: &str) -> Result<String> {
    let mut sub_messages = vec![
        ChatMessage::system(format!("You are a focused {agent_type} subagent.")),
        ChatMessage::user(prompt),
    ];
    // child loop uses base tools, then returns final text
}
```

Full source: `openai/src/lessons/lesson04_subagent.rs`. Entry point: `openai/src/bin/lesson04.rs`.

## Try It

```bash
cargo run -p ai-agent-learning-openai --bin lesson04 -- "派子 Agent 调查项目测试框架"
```

More prompts:

- 派子 Agent 阅读多个文件并总结
- 观察父上下文只增加一条摘要
