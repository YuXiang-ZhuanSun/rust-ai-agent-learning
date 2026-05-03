# AI Agent Learning (Rust Edition)

[中文版](README.md)

A progressive, hands-on tutorial for building AI agents from scratch in Rust. The course has 14 lessons built around readable Rust modules: OpenAI-compatible chat calls, tool dispatch, todo planning, skills, context compaction, background work, multi-agent messaging, protocols, autonomous task claiming, worktree isolation, and a final reference agent.

## Features

- 14 incremental lessons covering practical agent design patterns
- Pure Rust implementation with no heavyweight agent framework
- Direct OpenAI-compatible Chat Completions client built with `reqwest`
- File tools, TodoWrite, skill loading, task boards, background tasks, JSONL mailboxes, and worktree isolation
- Bilingual lesson docs and an interactive Next.js tutorial site
- Rust unit tests for the core runtime components

## Tech Stack

| Component | Version |
|-----------|---------|
| Rust | 1.75+ |
| Tokio | 1.x |
| Reqwest | 0.12 |
| Serde / serde_json | 1.x |
| Next.js (Web) | 16.1 |
| React (Web) | 19.2 |
| Tailwind CSS (Web) | 4 |

## Getting Started

```bash
git clone https://github.com/wukangxin/ai-agent-learning.git
cd ai-agent-learning
```

Set environment variables:

```bash
export OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxx
export OPENAI_MODEL=gpt-4o-mini
# optional: export OPENAI_BASE_URL=https://api.openai.com/v1
```

Build and test:

```bash
cargo build
cargo test
```

Run a lesson:

```bash
cargo run -p ai-agent-learning-openai --bin lesson00 -- "Hello, what can you do?"
cargo run -p ai-agent-learning-openai --bin lesson13 -- "Help me plan an executable project"
```

## Project Structure

```text
ai-agent-learning/
├── Cargo.toml
├── openai/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── openai.rs          # OpenAI-compatible client, messages, tool schemas
│   │   ├── runtime.rs         # Shared Todo, tasks, skills, messages, background work
│   │   ├── lessons/           # Lesson 00-13 source modules
│   │   └── bin/               # Runnable entry points
│   └── tests/                 # Rust unit tests
├── docs/openai/
│   ├── en/                    # English lesson docs
│   └── zh/                    # Chinese lesson docs
└── web/                       # Interactive tutorial website
```

## Lessons

| # | Topic | Description |
|---|-------|-------------|
| 0 | The Agent Loop | One bash tool plus tool-result feedback: the smallest agent |
| 1 | Tool Calling | Schema, arguments, handler, and tool result as one loop |
| 2 | Tool Dispatch | Register read/write/edit tools without changing the loop |
| 3 | TodoWrite | Turn plans into visible, updateable runtime state |
| 4 | Subagents | Delegate with isolated context and summary return |
| 5 | Skills | Cheap skill index plus on-demand SKILL.md loading |
| 6 | Context Compact | Micro compact, transcript save, and summary replacement |
| 7 | Task System | File-backed task board and dependency graph |
| 8 | Background Tasks | Run slow commands in the background and inject notifications |
| 9 | Agent Teams | Durable teammate identity, JSONL inboxes, async messages |
| 10 | Team Protocols | request_id tracking for approval and shutdown workflows |
| 11 | Autonomous Agents | Idle agents read messages, scan tasks, and claim work |
| 12 | Worktree Isolation | Task control plane plus isolated execution directories |
| 13 | Full Reference Agent | All mechanisms composed into one runnable reference agent |

## Web Tutorial Site

```bash
cd web
npm install
npm run dev
```

Then open http://localhost:3000/ai-agent-learning.

## Reference

- [wukangxin/ai-agent-learning](https://github.com/wukangxin/ai-agent-learning)

## License

This project is licensed under the [MIT License](LICENSE).

