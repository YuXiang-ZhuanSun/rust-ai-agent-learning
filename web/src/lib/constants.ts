export const LESSON_ORDER = [
  "L00", "L01", "L02", "L03", "L04", "L05", "L06",
  "L07", "L08", "L09", "L10", "L11", "L12", "L13",
] as const;

export const LEARNING_PATH = LESSON_ORDER;

export type LessonId = typeof LEARNING_PATH[number];

export const LESSON_META: Record<string, {
  title: string; titleZh: string; subtitle: string; subtitleZh: string;
  coreAddition: string; keyInsight: string; keyInsightZh: string;
  layer: "tools" | "planning" | "memory" | "concurrency" | "collaboration"; prevLesson: string | null;
}> = {
  L00: { title: "The Agent Loop", titleZh: "Agent 循环", subtitle: "One Tool, One Loop", subtitleZh: "一个工具，一个循环", coreAddition: "Bash tool + tool-result loop", keyInsight: "A coding agent is a feedback loop between model, tools, and observations.", keyInsightZh: "Coding Agent 的核心是模型、工具和观察结果之间的反馈循环。", layer: "tools", prevLesson: null },
  L01: { title: "Tool Calling", titleZh: "工具调用", subtitle: "Schema, Arguments, Result", subtitleZh: "Schema、参数、结果", coreAddition: "Typed tool schema and handler", keyInsight: "The model proposes actions; the host runtime executes them.", keyInsightZh: "模型提出动作，宿主运行时负责执行。", layer: "tools", prevLesson: "L00" },
  L02: { title: "Tool Dispatch", titleZh: "工具分发", subtitle: "Register Handlers", subtitleZh: "注册处理器", coreAddition: "Dispatch map for file and shell tools", keyInsight: "Adding tools should not change the loop.", keyInsightZh: "新增工具不应该改变循环本身。", layer: "tools", prevLesson: "L01" },
  L03: { title: "TodoWrite", titleZh: "待办管理", subtitle: "Visible Plan State", subtitleZh: "可见计划状态", coreAddition: "Todo manager and reminders", keyInsight: "Plans become runtime state the agent can inspect and update.", keyInsightZh: "计划成为 Agent 可检查、可更新的运行时状态。", layer: "planning", prevLesson: "L02" },
  L04: { title: "Subagents", titleZh: "子 Agent", subtitle: "Isolated Context", subtitleZh: "隔离上下文", coreAddition: "Fresh context delegation", keyInsight: "Delegation works when the child context is clean and the result is summarized.", keyInsightZh: "子上下文干净、结果可摘要，委托才可靠。", layer: "planning", prevLesson: "L03" },
  L05: { title: "Skills", titleZh: "技能系统", subtitle: "Load Knowledge On Demand", subtitleZh: "按需加载知识", coreAddition: "Two-layer skill loading", keyInsight: "Cheap indexes in the prompt, full knowledge in tool results.", keyInsightZh: "提示词里放轻量索引，工具结果里放完整知识。", layer: "planning", prevLesson: "L04" },
  L06: { title: "Context Compact", titleZh: "上下文压缩", subtitle: "Keep The Loop Alive", subtitleZh: "让循环持续", coreAddition: "Micro compact and summary replacement", keyInsight: "Compaction preserves state while removing low-value detail.", keyInsightZh: "压缩保留状态，移除低价值细节。", layer: "memory", prevLesson: "L05" },
  L07: { title: "Task System", titleZh: "任务系统", subtitle: "Durable Goals", subtitleZh: "持久目标", coreAddition: "File-backed task graph", keyInsight: "Tasks are the long-term control plane for agent work.", keyInsightZh: "任务板是 Agent 工作的长期控制平面。", layer: "planning", prevLesson: "L06" },
  L08: { title: "Background Tasks", titleZh: "后台任务", subtitle: "Non-blocking Work", subtitleZh: "非阻塞工作", coreAddition: "Background threads and notifications", keyInsight: "Slow work can finish later without freezing the agent loop.", keyInsightZh: "慢任务可以稍后完成，而不冻结 Agent 循环。", layer: "concurrency", prevLesson: "L07" },
  L09: { title: "Agent Teams", titleZh: "Agent 团队", subtitle: "Mailboxes", subtitleZh: "邮箱通信", coreAddition: "Teammates and JSONL inboxes", keyInsight: "Teamwork needs durable identity and inspectable communication.", keyInsightZh: "团队协作需要持久身份和可检查通信。", layer: "collaboration", prevLesson: "L08" },
  L10: { title: "Team Protocols", titleZh: "团队协议", subtitle: "Correlated Requests", subtitleZh: "关联请求", coreAddition: "request_id protocol tracking", keyInsight: "Protocols are messages plus correlation and state.", keyInsightZh: "协议就是消息、关联 ID 和状态追踪。", layer: "collaboration", prevLesson: "L09" },
  L11: { title: "Autonomous Agents", titleZh: "自治 Agent", subtitle: "Idle Policy", subtitleZh: "空闲策略", coreAddition: "Inbox polling and task claiming", keyInsight: "Autonomy appears in the idle loop: read, scan, claim, resume.", keyInsightZh: "自治出现在空闲循环：读消息、扫任务、认领、继续。", layer: "collaboration", prevLesson: "L10" },
  L12: { title: "Worktree Isolation", titleZh: "工作树隔离", subtitle: "Separate Execution Lanes", subtitleZh: "隔离执行通道", coreAddition: "Task-bound worktree lanes", keyInsight: "Coordinate by task id; isolate by directory.", keyInsightZh: "按任务 ID 协调，按目录隔离。", layer: "collaboration", prevLesson: "L11" },
  L13: { title: "Full Reference Agent", titleZh: "完整参考 Agent", subtitle: "Mechanisms Composed", subtitleZh: "机制组合", coreAddition: "All mechanisms in one loop", keyInsight: "A complete agent is careful state orchestration around the same loop.", keyInsightZh: "完整 Agent 是围绕同一个循环做精细状态编排。", layer: "collaboration", prevLesson: "L12" },
};

export const LAYERS = [
  { id: "tools" as const, label: "Tools & Execution", labelZh: "工具与执行", color: "#2563EB", lessons: ["L00", "L01", "L02"] },
  { id: "planning" as const, label: "Planning & Coordination", labelZh: "规划与协调", color: "#059669", lessons: ["L03", "L04", "L05", "L07"] },
  { id: "memory" as const, label: "Memory Management", labelZh: "记忆管理", color: "#7C3AED", lessons: ["L06"] },
  { id: "concurrency" as const, label: "Concurrency", labelZh: "并发", color: "#D97706", lessons: ["L08"] },
  { id: "collaboration" as const, label: "Collaboration", labelZh: "协作", color: "#DC2626", lessons: ["L09", "L10", "L11", "L12", "L13"] },
] as const;
