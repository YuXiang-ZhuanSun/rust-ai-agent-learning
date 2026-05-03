# Lesson 2: 工具分发

```text
L00 > L01 > [ L02 ] > L03 > L04 > L05 > L06 > L07 > L08 > L09 > L10 > L11 > L12 > L13
```

> Agent 能力扩展靠注册工具，不靠改循环。

## 问题

只有 bash 时，所有行为都挤进 shell：读文件、写文件、替换文本都靠字符串命令。这样不可控，也难测试。工具分发把“模型想做什么”拆成多个窄接口：read_file 只读，write_file 只写，edit_file 只做精确替换。循环不需要知道每个工具细节，只按名字分发。

## 解决方案

```text
tool name
   |
   v
ToolRegistry {
  bash -> run_shell
  read_file -> fs::read_to_string
  write_file -> fs::write
  edit_file -> exact replace
}
```

## 工作原理

1. 每个工具有自己的 schema 和 handler。
2. `ToolRegistry` 保存工具名到函数指针的映射。
3. 路径工具先做 workspace sandbox 检查，禁止绝对路径和 `..`。
4. 循环只调用 `registry.execute(name, args)`。
5. 新增工具时，只增加 schema + handler，不碰主循环。

## 本章机制

| 组件 | 作用 |
|------|------|
| read_file | 稳定读取文本，支持后续增加 limit |
| write_file | 创建父目录并写入内容 |
| edit_file | 精确替换，找不到旧文本就返回错误 |
| sandbox | 把文件能力限制在工作区内 |

## 源码切片

```rust
pub fn execute(&self, name: &str, args: Value) -> Result<String> {
    let handler = self.handlers.get(name)
        .ok_or_else(|| anyhow!("unknown tool: {name}"))?;
    handler(&self.root, args)
}
```

完整源码：`openai/src/lessons/lesson02_tools.rs`。运行入口：`openai/src/bin/lesson02.rs`。

## 试一试

```bash
cargo run -p ai-agent-learning-openai --bin lesson02 -- "让 Agent 创建并读取一个文件"
```

还可以试：

- 尝试读取 ../secret.txt，确认被拦截
- 新增 list_files 工具
