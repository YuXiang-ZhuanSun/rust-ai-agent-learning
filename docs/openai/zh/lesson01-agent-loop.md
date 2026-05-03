# Lesson 1: 工具调用

```text
L00 > [ L01 ] > L02 > L03 > L04 > L05 > L06 > L07 > L08 > L09 > L10 > L11 > L12 > L13
```

> 工具调用不是“函数语法”，而是模型和宿主运行时之间的行动协议。

## 问题

模型不能直接执行动作，它只能提出一个结构化请求：工具名是什么，参数是什么。宿主程序决定是否接受、如何执行、怎样把结果返回。学习工具调用时，重点不是工具本身有多复杂，而是 schema、arguments、handler、tool_result 这四个位置必须闭合。

## 解决方案

```text
ToolSpec(schema)
     |
     v
LLM returns { name, arguments }
     |
     v
Rust handler validates + runs
     |
     v
tool_result goes back into messages
```

## 工作原理

1. 用 `ToolSpec` 描述工具名、用途和 JSON 参数结构。
2. 模型根据用户问题决定是否调用工具。
3. Rust 不信任模型参数，先解析 JSON，再交给 handler。
4. handler 返回的是观察结果，不是最终答案。
5. 模型再根据观察结果组织最终回复。

## 本章机制

| 组件 | 作用 |
|------|------|
| Schema | 告诉模型“你能请求什么” |
| Arguments | 模型生成的结构化行动意图 |
| Handler | 宿主程序里的可信执行逻辑 |
| Result | 把执行结果重新交给模型推理 |

## 源码切片

```rust
pub fn weather_tool() -> ToolSpec {
    ToolSpec::object("get_weather", "Return weather for a city", properties, &["city"])
}

if let Some(calls) = &reply.tool_calls {
    for call in calls {
        let output = execute_weather(&call.function.arguments);
        messages.push(ChatMessage::tool(&call.id, output));
    }
}
```

完整源码：`openai/src/lessons/lesson01_tool_use.rs`。运行入口：`openai/src/bin/lesson01.rs`。

## 试一试

```bash
cargo run -p ai-agent-learning-openai --bin lesson01 -- "问一个需要 get_weather 的问题"
```

还可以试：

- 把工具返回改成错误，观察模型如何恢复
- 新增一个 echo 工具并注册
