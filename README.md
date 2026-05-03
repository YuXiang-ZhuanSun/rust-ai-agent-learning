# AI Agent 学习教程（Rust 版）

[English](README_en.md)

一个渐进式、实践导向的 AI Agent 构建教程，使用 Rust 从零实现 Agent 的核心机制。课程共 14 节，从最小聊天请求开始，逐步加入工具调用、任务规划、技能加载、上下文压缩、后台任务、多 Agent 协作、协议和工作目录隔离，最终组合成一个完整参考 Agent。

## 特性

- 14 节渐进式课程，覆盖 Agent 工程里的主要设计模式
- 纯 Rust 实现，不依赖重量级 Agent 框架
- 直接调用 OpenAI 兼容 Chat Completions API，便于理解 HTTP、消息和工具调用结构
- 文件系统工具、TodoWrite、技能加载、任务板、后台任务、JSONL 消息总线和工作树隔离
- 保留中英文课程文档与 Next.js 交互式教程网站
- Rust 单元测试覆盖核心运行时组件

## 技术栈

| 组件 | 版本 |
|------|------|
| Rust | 1.75+ |
| Tokio | 1.x |
| Reqwest | 0.12 |
| Serde / serde_json | 1.x |
| Next.js（Web） | 16.1 |
| React（Web） | 19.2 |
| Tailwind CSS（Web） | 4 |

## 快速开始

```bash
git clone https://github.com/wukangxin/ai-agent-learning.git
cd ai-agent-learning
```

设置环境变量：

```bash
export OPENAI_API_KEY=sk-xxxxxxxxxxxxxxxxxxxx
export OPENAI_MODEL=gpt-4o-mini
# 可选：export OPENAI_BASE_URL=https://api.openai.com/v1
```

构建和测试：

```bash
cargo build
cargo test
```

运行课程：

```bash
cargo run -p ai-agent-learning-openai --bin lesson00 -- "你好，你能做什么？"
cargo run -p ai-agent-learning-openai --bin lesson13 -- "帮我规划一个可执行项目"
```

## 项目结构

```text
ai-agent-learning/
├── Cargo.toml
├── openai/
│   ├── Cargo.toml
│   ├── src/
│   │   ├── openai.rs          # OpenAI 兼容客户端、消息和工具 schema
│   │   ├── runtime.rs         # Todo、任务、技能、消息、后台任务等共享组件
│   │   ├── lessons/           # Lesson 00-13 的渐进源码
│   │   └── bin/               # 每节课的可运行入口
│   └── tests/                 # Rust 单元测试
├── docs/openai/
│   ├── en/                    # 英文课程文档
│   └── zh/                    # 中文课程文档
└── web/                       # 交互式教程网站
```

## 课程列表

| # | 主题 | 描述 |
|---|------|------|
| 0 | Agent 循环 | 一个 bash 工具 + tool_result 回填循环，形成最小 Agent |
| 1 | 工具调用 | schema、arguments、handler、tool result 的完整闭环 |
| 2 | 工具分发 | 用注册表扩展 read/write/edit 等能力，循环不变 |
| 3 | TodoWrite | 把计划变成可见、可更新的运行时状态 |
| 4 | 子 Agent | 用独立上下文处理委托任务，只返回摘要 |
| 5 | 技能系统 | 轻量技能索引 + 按需加载 SKILL.md 正文 |
| 6 | 上下文压缩 | 微压缩、保存 transcript、摘要替换历史 |
| 7 | 任务系统 | 文件持久化任务板与依赖图 |
| 8 | 后台任务 | 慢命令后台执行，完成后通过通知回到循环 |
| 9 | Agent 团队 | 持久队友身份、JSONL inbox 和异步消息 |
| 10 | 团队协议 | 用 request_id 跟踪审批、关机等协作协议 |
| 11 | 自治 Agent | 空闲阶段读消息、扫任务、主动认领工作 |
| 12 | 工作树隔离 | 任务控制平面 + 独立目录执行平面 |
| 13 | 完整参考 Agent | 把所有机制组合进一个可运行参考 Agent |

## 本地运行网站

```bash
cd web
npm install
npm run dev
```

然后打开 http://localhost:3000/ai-agent-learning。

## 参考致谢

- [wukangxin/ai-agent-learning](https://github.com/wukangxin/ai-agent-learning)

## 许可证

本项目基于 [MIT License](LICENSE) 开源。

