## Context

当前 `open` 动作(`yazi-actor/src/mgr/open.rs`)对所有目标统一处理: 获取 MIME 类型后交给 `open_do` 用关联程序打开.  而 `enter` 动作(`enter.rs`)检查 hovered 项是否为目录, 是则调用 `cd` 进入.

用户希望 Enter 键(绑定到 `open`)在目标为目录时自动进入, 无需改键位.

## Goals / Non-Goals

**Goals:**
- `open` 动作在目标为单个目录时, 自动转为 `enter` 行为(进入目录)
- 文件的 `open` 行为完全不变

**Non-Goals:**
- 不修改键位映射(keymap-default.toml)
- 不修改 `enter`/`leave` 动作本身
- 不修改 `open --interactive` 行为(交互模式仍正常弹出 picker)

## Decisions

### 在 `Open::act` 中提前判断目录类型

**方案**: 在 `open.rs` 的 `act` 方法中, 收集完 targets 后, 检查是否仅有一个目标且为目录.  若是, 直接调用 `act!(mgr:enter, cx)` 并返回, 跳过后续的 MIME 类型获取和 `open_do` 流程.

**替代方案**: 在 keymap 中用条件表达式替换 Enter 的绑定.  但 yazi 的 keymap 不支持条件逻辑, 且这样会把行为逻辑放到配置层, 不利于维护.

**理由**: 修改 `open` 动作更符合语义 — "打开目录"就是"进入目录", 且改动集中在一处.

### 仅对 hovered 单目录生效

当选中多个文件/目录混合时, 保持原有行为(用关联程序打开). 目录进入逻辑仅在 hovered 单个目录时触发, 与 `enter` 动作的行为一致.

## Risks / Trade-offs

- [用户依赖旧行为] → 影响极小, 用编辑器打开目录本身不是常见需求; 如需仍可用 `open --interactive` 或直接配置键位
- [多文件选中含目录] → 保持原有行为, 不做特殊处理, 避免引入复杂逻辑
