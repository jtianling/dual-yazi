## Context

当前 `keymap-default.toml` 中 tab 操作使用组合键: `tt` 创建 tab, `tr` 重命名 tab.  这与原版 yazi 的默认键绑定不一致 — 原版用单键 `t` 创建 tab, 且默认不绑定 tab_rename.  `r` 键在原版中是 rename 文件, 当前配置中也已正确绑定.

## Goals / Non-Goals

**Goals:**
- 恢复 `t` 单键创建 tab 的行为, 与原版 yazi 一致
- 移除 `tr` 组合键的 tab_rename 绑定

**Non-Goals:**
- 不改动任何 Rust 代码, 仅修改 keymap 配置
- 不为 tab_rename 分配新的默认键绑定(用户可自行配置)

## Decisions

1. **将 `tab_create` 从 `["t", "t"]` 改为 `"t"`**: 与原版 yazi 行为一致, 减少击键次数. 无替代方案需要考虑.
2. **直接移除 `["t", "r"]` 绑定**: 原版 yazi 默认不绑定 tab_rename, `tab_rename` 功能仍然存在, 用户可通过自定义 keymap 绑定.

## Risks / Trade-offs

- [风险] 已习惯 `tt`/`tr` 的用户需要适应 → 影响面极小, 项目仍在开发阶段
- [风险] tab_rename 没有默认键绑定 → 用户可通过自定义 keymap 配置, 与原版 yazi 保持一致
