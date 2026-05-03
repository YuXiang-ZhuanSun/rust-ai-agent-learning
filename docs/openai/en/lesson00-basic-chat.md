# Lesson 0: The Agent Loop

```text
[ L00 ] > L01 > L02 > L03 > L04 > L05 > L06 > L07 > L08 > L09 > L10 > L11 > L12 > L13
```

> 一个 bash 工具 + 一个 while 循环，就是最小可用的 Coding Agent。

## Problem

LLM 会推理，但不会自己碰文件系统。它不知道 `cargo test` 的真实输出，也不能替你创建文件。Agent Loop 的意义，就是把模型的“下一步意图”变成宿主程序执行的工具调用，再把观察结果放回消息历史。没有这个循环，工具调用只是一次性函数；有了循环，模型可以根据真实反馈继续行动。

## Solution

```text
+--------+      +-------+      +-------------+
| User   | ---> |  LLM  | ---> | bash tool   |
| goal   |      |       |      | executes    |
+--------+      +---+---+      +------+------+
                    ^                 |
                    |   tool_result   |
                    +-----------------+
              loop until no tool calls
```

## How It Works

1. 用户输入先进入 `messages`，这是 Agent 当前任务的工作记忆。
2. 系统提示只规定角色和边界：你是 coding agent，可以使用 bash。
3. 模型如果返回工具调用，Rust 解析参数并执行 `run_bash`。
4. 命令输出不直接打印完事，而是作为 `tool` 消息回填给模型。
5. 模型看到真实输出后继续下一轮，直到它不再调用工具。

## Mechanism

| Component | Role |
|-----------|------|
| 工具 | `bash`：让模型第一次接触真实世界 |
| 状态 | `messages: Vec<ChatMessage>` 累积所有观察 |
| 安全 | 危险命令拦截 + 最大轮数限制 |
| 核心 | 工具结果必须回填给模型，而不是只给用户看 |

## Source Slice

```rust
pub async fn agent_loop(client: &OpenAiClient, messages: &mut Vec<ChatMessage>) -> Result<()> {
    let tools = vec![bash_tool()];
    for _ in 0..MAX_ROUNDS {
        let response = client.chat(messages, &tools).await?;
        let tool_calls = response.tool_calls.clone().unwrap_or_default();
        messages.push(response);
        if tool_calls.is_empty() { return Ok(()); }
        for call in tool_calls {
            let output = run_bash(command_from(&call)?);
            messages.push(ChatMessage::tool(call.id, output));
        }
    }
    Ok(())
}
```

Full source: `openai/src/lessons/lesson00_basic_chat.rs`. Entry point: `openai/src/bin/lesson00.rs`.

## Try It

```bash
cargo run -p ai-agent-learning-openai --bin lesson00 -- "创建一个 hello.txt 并写入一句话"
```

More prompts:

- 列出当前目录下的 Rust 文件
- 运行一次 cargo test 并解释失败原因
