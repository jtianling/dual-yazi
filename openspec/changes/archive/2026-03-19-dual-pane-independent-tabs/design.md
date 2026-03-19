## Context

当前双面板实现使用扁平的 `Vec<Tab>` 加单一 `cursor`, 硬编码恰好 2 个 tab 分别代表左右面板.  `tab_create`/`tab_close`/`tab_switch` 等操作被早返回守卫禁用.  `Tabs::Default` 固定创建 2 个 tab.

原版 yazi 支持任意数量的 tab, 用户可自由创建/关闭/切换.  本次变更需要在保留双面板的前提下, 恢复每个面板内独立的多 tab 功能.

## Goals / Non-Goals

**Goals:**
- 每个面板维护独立的 tab 列表和 cursor
- 恢复所有 tab 管理操作 (create/close/switch/swap/rename), 作用于当前活跃面板
- 恢复 keymap 中原有的 tab 管理键位
- 跨面板操作 (copy_to/move_to) 以对面面板的活跃 tab 为目标
- 在面板内有多 tab 时显示 tab bar

**Non-Goals:**
- 不支持跨面板移动 tab (将 tab 从左面板拖到右面板)
- 不改变单面板模式的 tab 行为 (单面板模式下行为与原版一致)
- 不支持面板间不同的 tab bar 样式定制

## Decisions

### Decision 1: Tabs 结构改为 Pane 包含 TabGroup

**方案**: 引入 `Pane` 结构, 包含 `tabs: Vec<Tab>` 和 `cursor: usize`.  `Mgr` 持有 `panes: [Pane; 2]` 和 `active_pane: usize`.

**替代方案**: 在现有 `Tabs` 上用分区索引模拟两组 tab — 复杂度高, 容易出错.

**理由**: `Pane` 结构清晰地表达 "每个面板是独立的 tab 集合" 语义, 与原版 `Tabs` 的 `Vec<Tab> + cursor` 模式一致, 只是从一组变两组.  tab 操作代码几乎不需要改逻辑, 只需要改作用域 (从全局 tabs 变为 active pane 的 tabs).

### Decision 2: 保持 Actor 接口不变

**方案**: `tab_create`/`tab_close` 等 Actor 的函数签名和 Options 不变, 内部从 `cx.tabs_mut()` 改为 `cx.pane_mut().tabs` 或等效访问.

**理由**: 最小化变更, Parser 层完全不需要改动, Spark 枚举也不需要改.

### Decision 3: Ctx 适配方法

**方案**: 在 `Ctx` 上提供 `active_pane()`, `other_pane()`, `pane_mut()` 等便捷方法.  现有的 `cx.tab()`, `cx.tabs()` 等方法改为代理到当前活跃面板的 tab.

**理由**: 保持大量现有代码 (`cx.tab()`, `cx.cwd()` 等) 的兼容性, 减少需要修改的文件数量.

### Decision 4: Tab bar 在面板内条件显示

**方案**: 当某个面板的 tab 数量 > 1 时, 在该面板的 Header 和内容区之间显示 tab bar.  只有 1 个 tab 时不显示.

**理由**: 与原版 yazi 行为一致 (单 tab 不显示 tab bar), 最大化利用屏幕空间.

### Decision 5: update_files / watch 遍历所有面板的所有 tab

**方案**: `update_files` 和 `watch` 遍历两个面板的所有 tab (而非仅 2 个 tab).

**理由**: 确保文件变更通知到达所有打开的 tab, 无论在哪个面板.

## Risks / Trade-offs

**[Risk] Ctx 兼容层可能遗漏某些调用点** → 全局搜索 `cx.tabs()`, `cx.tab()`, `cx.mgr.tabs` 等模式, 确保所有访问点都通过兼容层.

**[Risk] 面板内 tab 数量过多导致 tab bar 占用过多空间** → 与原版 yazi 相同的问题, 不在本次变更范围内解决.

**[Trade-off] `Pane` 结构 vs 保持扁平 `Tabs`** → 选择新结构牺牲了一些向后兼容性, 但获得了清晰的语义和更简单的实现.
