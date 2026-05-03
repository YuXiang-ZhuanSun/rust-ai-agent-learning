# Lesson 5: Skills

```text
L00 > L01 > L02 > L03 > L04 > [ L05 ] > L06 > L07 > L08 > L09 > L10 > L11 > L12 > L13
```

> 知识要按需加载，不要全塞进 system prompt。

## Problem

Agent 需要领域知识：发布流程、代码审查清单、框架约定。但如果每次请求都把所有知识放进系统提示，token 浪费巨大，也会稀释注意力。技能系统把知识拆成两层：提示词里只放技能索引，真正需要时才加载完整 SKILL.md。

## Solution

```text
System prompt: skill index
  - rust-review: ...
  - release: ...
        | load_skill("release")
        v
tool_result: <skill> full markdown body </skill>
```

## How It Works

1. 启动时递归扫描 `skills/**/SKILL.md`。
2. 解析 frontmatter 中的 name、description 等元信息。
3. 系统提示只注入技能名称和一句话描述。
4. 模型遇到陌生任务时调用 `load_skill`。
5. 完整技能正文作为 tool_result 注入当前对话。

## Mechanism

| Component | Role |
|-----------|------|
| Layer 1 | 低成本技能索引 |
| Layer 2 | 按需加载完整正文 |
| 格式 | SKILL.md + frontmatter + Markdown |
| 收益 | 减少上下文常驻知识 |

## Source Slice

```rust
let loader = SkillLoader::from_dir(workspace.join("skills"))?;
let system = skill_system_prompt(&loader.descriptions());
// later
let body = loader.load("release")?;
messages.push(ChatMessage::tool(call.id, body));
```

Full source: `openai/src/lessons/lesson05_skills.rs`. Entry point: `openai/src/bin/lesson05.rs`.

## Try It

```bash
cargo run -p ai-agent-learning-openai --bin lesson05 -- "创建一个 skills/release/SKILL.md"
```

More prompts:

- 让 Agent 先加载技能再执行发布检查
- 对比加载前后的回答质量
