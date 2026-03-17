## Context

Yazi 是一个基于 Rust + Lua 的终端文件管理器, 采用 monorepo 架构 (20+ crates).  核心数据结构为 `Core → Mgr → Tabs(Vec<Tab>) → Tab(current + parent + preview)`.  UI 通过 Lua 组件树渲染 (ratatui backend), 当前布局为单栏 miller columns (parent + current + preview).

现有 Tabs 系统支持最多 9 个 tab, 通过 `tabs.cursor` 指向活跃 tab.  Action 系统通过 `Ctx` 上下文执行, `Ctx.tab` 索引确定操作目标.  Lua 渲染通过 `Lives::scope()` 将 Rust 状态暴露为 `cx` 全局变量.

## Goals / Non-Goals

**Goals:**
- 将 yazi 改造为双栏文件管理器, 两个 pane 并排显示
- 支持 Tab/Ctrl-w 系列快捷键切换活跃栏
- 支持 MC 风格 F5/F6 直接跨栏复制/移动 (不经过 yank 寄存器)
- 双栏模式下默认隐藏 preview 列, 每栏显示 parent + current
- 保持现有 yank/paste 流程可用 (在另一栏 paste 时自动以该栏 cwd 为目标)

**Non-Goals:**
- 不支持动态创建/关闭 pane (固定两栏)
- 不支持 :split / :vsplit 切换水平/垂直分栏 (仅垂直分栏)
- 不支持 :only 回到单栏模式
- 不支持 :sync 同步两栏目录
- 不支持 pane 大小调整 (Ctrl-w =/</>)
- 不处理多 tab 兼容 (完全移除 tab 概念)

## Decisions

### D1: 复用 Tabs 数据结构作为 Panes

**决策**: 不新建 Panes 类型, 而是将现有 `Tabs` 约束为固定包含 2 个 `Tab`, `cursor` 作为活跃栏索引 (0=left, 1=right).

**理由**: Tab 和 Pane 在数据层面完全同构 (都是 Folder + Preview + Selected + History 的容器).  复用避免了大量重复代码和 Lua bridge 改动.

**替代方案**: 新建 `Panes { left: Tab, right: Tab, active: usize }` - 语义更清晰但需要在整个 actor/lives/executor 链路中增加并行的 Pane 处理路径, 工作量大且收益低.

### D2: MC 风格直接操作 vs 扩展 yank/paste

**决策**: F5/F6 实现为独立的 `copy_to` / `move_to` action, 直接获取选中文件和另一栏 cwd 进行操作, 不经过 yank 寄存器.

**理由**: 这与 vifm 的实现一致 (`fops_cpmv()` 直接用 `other_view->curr_dir`).  yank/paste 流程保持不变, 两套操作并存: yank+switch+paste 适合精确控制, F5/F6 适合快速批量操作.

**替代方案**: 扩展 PasteOpt 添加 `--to-other` flag - 更简单但语义不如独立 action 清晰, 且 F5 需要跳过 yank 步骤.

### D3: 双栏默认 ratio

**决策**: 双栏模式下每个 pane 使用 `ratio = [1, 2, 0]`, 隐藏 preview 列.

**理由**: 两个完整 miller (各 3 列 = 6 列) 在普通终端宽度下过于拥挤.  `[1, 2, 0]` 是 yazi ratio 系统已支持的合法值, 无需改动 ratio 解析逻辑.

### D4: Lua UI 改造策略

**决策**: 在 `root.lua` 中用 `DualPane` 组件替代 `Tabs` bar + 单个 `Tab` 渲染.  `DualPane` 将区域水平分为 left + separator + right, 分别实例化两个 `Tab` 组件.

**理由**: Tab 组件已经支持接收任意 tab 对象参数 (`Tab:new(area, tab)`), 只需传入 `cx.tabs[1]` 和 `cx.tabs[2]` 即可.  无需改动 Tab/Parent/Current/Preview 内部渲染逻辑.

### D5: 禁用 tab 管理 action 的方式

**决策**: 在 `Mgr::make()` 中初始化 2 个 Tab, 在 `tab_create`/`tab_close` 等 action 中加入前置检查, 当处于双栏模式时直接返回 (no-op).

**理由**: 完全删除 tab action 代码风险太大, 可能破坏其他依赖.  使检查静默失败更安全.

## Risks / Trade-offs

- **窄终端体验**: 两栏各含 parent+current, 在 < 80 列终端可能过窄 → 后续可添加自动隐藏 parent 的逻辑, 本阶段不处理
- **插件兼容性**: `cx.active` 仍指向活跃 tab, `cx.tabs` 仍然可用, 大多数插件不受影响 → 但依赖 tab 数量或 tab bar 的插件可能需要适配
- **Watcher/Pubsub**: 两个 tab 意味着同时 watch 两个目录, 确认 `Watcher` 支持多目录 → 现有实现已支持 (每个 tab 独立 watch)
- **Preview 缺失**: 默认关闭 preview 会降低文件浏览体验 → 用户可通过 yazi 配置文件将 ratio 改为 `[1, 2, 1]` 恢复
