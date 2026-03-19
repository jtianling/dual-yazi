## Why

当前 Enter 键对文件和目录都执行 `open` 动作(用编辑器打开), 但在文件管理器中, 对目录按 Enter 更符合直觉的操作是进入该目录, 而非用编辑器打开.  这与大多数文件管理器(如 Midnight Commander, ranger)的行为一致.

## What Changes

- 修改 `open` 动作: 当目标为目录时, 执行 `enter`(进入目录)而非用编辑器打开
- 文件的 `open` 行为保持不变, 仍然用关联程序/编辑器打开
- 键位映射不变, Enter 仍然绑定到 `open`

## Capabilities

### New Capabilities

- `open-dir-enter`: 当 `open` 动作的目标为目录时, 自动转为 `enter` 动作进入该目录

### Modified Capabilities

(无)

## Impact

- 影响文件: `yazi-actor/src/mgr/open.rs` — `open` 动作入口, 需在此判断目标类型
- 不影响 `open --interactive` 行为(交互模式仍正常打开选择器)
- 不影响 `enter`/`leave` 等其他导航动作
- 不影响键位配置
