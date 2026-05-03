# Lesson 6: Context Compact

```text
L00 > L01 > L02 > L03 > L04 > L05 > [ L06 ] > L07 > L08 > L09 > L10 > L11 > L12 > L13
```

> Agent 要能工作很久，就必须学会有策略地遗忘。

## Problem

工具结果、文件内容、命令输出会不断进入消息列表。上下文窗口再大也会满。压缩不是简单删除历史，而是把低价值细节替换成高价值状态：目标、已完成事项、当前文件、关键决策、未解决问题。

## Solution

```text
every turn -> micro compact old tool results
         -> if token estimate high
         -> save transcript
         -> replace history with summary
manual compress tool -> same pipeline
```

## How It Works

1. 每轮调用前，先截断较旧的 tool result。
2. 估算 tokens，超过阈值时触发 auto compact。
3. 压缩前把完整 transcript 保存到 `.transcripts/`。
4. 用摘要消息替换旧历史，让循环继续。
5. 模型也可以主动调用 `compress` 请求压缩。

## Mechanism

| Component | Role |
|-----------|------|
| micro compact | 保留近期结果，压缩旧结果 |
| auto compact | 超过阈值自动摘要 |
| transcript | 完整历史落盘，可审计可恢复 |
| manual compact | 模型主动清理上下文 |

## Source Slice

```rust
micro_compact(&mut messages, 8, 500);
if estimate_tokens(&messages) > self.threshold {
    self.save_transcript(round, &messages)?;
    messages = summary_replacement("preserve goal, state, open tasks");
}
```

Full source: `openai/src/lessons/lesson06_context_compact.rs`. Entry point: `openai/src/bin/lesson06.rs`.

## Try It

```bash
cargo run -p ai-agent-learning-openai --bin lesson06 -- "让 Agent 连续读取多个大文件"
```

More prompts:

- 手动调用 compress 工具
- 检查 .transcripts 是否保存历史
