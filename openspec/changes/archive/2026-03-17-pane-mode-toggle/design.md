## Context

dual-yazi 已通过 dual-pane-mode 变更实现了固定双栏布局.  `Tabs` 结构包含 2 个 `Tab`, `tabs.cursor` 指向活跃栏.  Lua 层 `root.lua` 使用 `DualPane` 组件渲染两栏, 每栏 ratio 为 `[1, 2, 0]` (无 preview).

用户需要在双栏和单栏之间切换: 单栏模式下只显示活跃 pane 并恢复 preview, 再次切换回双栏.

## Goals / Non-Goals

**Goals:**
- 提供 `Ctrl-w o` toggle 在双栏和单栏之间切换
- 单栏模式: 只渲染活跃 pane, ratio 恢复为带 preview 的配置
- 非活跃 pane 状态完整保留 (目录, 选中, 历史等), 切回双栏时无缝恢复
- Lua 层通过检查模式状态决定渲染 DualPane 还是单 Tab

**Non-Goals:**
- 不支持独立配置单栏和双栏各自的 ratio (使用统一配置, 单栏模式用原始默认值)
- 不支持记忆上次退出时的模式 (每次启动固定双栏)
- 不支持 :only 命令行模式 (仅快捷键)

## Decisions

### D1: 状态存储位置

**决策**: 在 `Tabs` 结构中新增 `pub single_pane: bool` 字段, 默认 `false`.

**理由**: 这是最轻量的改动.  `Tabs` 已经是 pane 管理的核心结构, 布局模式本质上是 pane 管理的一部分.  通过 Lua bridge 直接暴露为 `cx.tabs.single_pane` 即可驱动 UI 切换.

**替代方案**: 在 `Mgr` 上加字段 - 语义没问题但需要额外的 lives 暴露; 在 `Core` 上加字段 - 层级太高, 不应感知布局细节.

### D2: Ratio 切换策略

**决策**: 不动态修改 ratio 配置.  而是在 Lua 渲染层处理:
- 双栏模式: DualPane 组件使用硬编码的 `[1, 2, 0]` ratio (或读取配置中的 dual-pane ratio)
- 单栏模式: 直接使用 `rt.mgr.ratio` 配置值 (用户配置的原始 ratio, 默认 `[1, 4, 3]`)

**理由**: 避免在 Rust 端来回切换 ratio 配置值.  Lua 层本来就负责布局, 它可以根据 `cx.tabs.single_pane` 选择不同的布局参数.  这样用户配置的 ratio 始终代表"单栏时的偏好", 双栏 ratio 由 DualPane 组件内部决定.

### D3: 快捷键选择

**决策**: 使用 `Ctrl-w o` 作为 toggle 快捷键.

**理由**:
- 与 vim 的 `Ctrl-w o` (`:only`) 语义一致, vi 用户天然理解
- 与 vifm 的 `:only` / `Ctrl-w o` 一致
- 不与已有绑定冲突 (Ctrl-w h/l/w 已用于 pane 切换)
- Toggle 行为 (而非 vifm 的单向 `:only`) 更实用, 因为 dual-yazi 没有动态创建 pane 的机制

### D4: 单栏模式下的 pane_switch / copy_to / move_to 行为

**决策**: 单栏模式下 `pane_switch` 仍然切换 `tabs.cursor`, 但 UI 只渲染活跃 pane.  `copy_to` / `move_to` 仍然工作 (目标是隐藏的另一栏的 cwd).

**理由**: 保持操作一致性.  用户可以在单栏模式下 F5 复制到另一栏的目录, 即使看不到另一栏.  这与 vifm 的 `:only` 模式行为一致 (隐藏的 pane 仍然存在并可操作).

## Risks / Trade-offs

- **UX 可发现性**: 用户可能不知道 `Ctrl-w o` 的存在 → 可在 help 中说明; 后续可考虑状态栏显示当前模式
- **单栏下跨栏操作不直观**: F5 复制到一个看不见的目录 → 可接受, 与 vifm 行为一致, 用户可先切回双栏再操作
