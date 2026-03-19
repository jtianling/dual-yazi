## 1. 核心实现

- [x] 1.1 修改 `yazi-actor/src/mgr/open.rs` 的 `Open::act` 方法: 在收集完 targets 后, 判断是否为非交互模式且仅有一个目标且该目标为目录, 若是则调用 `act!(mgr:enter, cx)` 并返回

## 2. 测试验证

- [x] 2.1 TUI 功能测试: 启动 yazi, 在目录上按 Enter 验证进入目录; 在文件上按 Enter 验证用编辑器打开
- [x] 2.2 编译检查: `cargo clippy` 确认无警告
