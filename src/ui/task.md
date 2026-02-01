# UI Module Implementation Tasks

## 实际完成情况摘要

**最后更新日期**: 2026-02-01

**真实完成度**: 约 75%

**主要缺失功能**:
- Phase 5.3-5.4: 转会市场购买/出售功能 - 有TODO，未完全实现
- Phase 6.2-6.5: 比赛相关界面 - 部分有TODO
- Phase 8.1: 存档/读取界面 - 有TODO，未完全实现

**已完成模块**:
- Phase 1-4: 基础框架、主菜单、球队管理、战术设置 - 完整实现
- Phase 7: 联赛积分榜 - 完整实现
- Phase 9-13: 组件、样式、输入处理、集成 - 基本实现

---

## 重要说明

**国际化要求**: 所有界面的 `render()` 方法必须接收 `i18n: &I18n` 参数，并使用 `i18n.t(TranslationKey::...)` 获取翻译文本。禁止在界面代码中硬编码中文字符串。

## Phase 1: 基础框架

### Task 1.1: 项目依赖
- [x] 添加 `ratatui` 依赖
- [x] 添加 `crossterm` 依赖
- [x] 配置 Cargo.toml

**Acceptance Criteria**: `cargo build` 成功

---

### Task 1.2: App 结构
- [x] 定义 `App` struct
- [x] 在 `App` 中添加 `i18n: I18n` 字段
- [x] 实现 `App::new()`，支持语言参数
- [x] 实现语言检测逻辑
- [x] 实现 `run()` 方法框架
- [x] 更新 `render()` 方法，传递 `i18n` 给 Screen

**Acceptance Criteria**: App 可以创建和运行，包含国际化支持

---

### Task 1.3: Screen Trait
- [x] 定义 `Screen` trait
- [x] 定义所有必需方法

**Acceptance Criteria**: Trait 定义完整

---

### Task 1.4: 国际化支持
- [x] 创建 `i18n.rs` 文件
- [x] 定义 `Language` 枚举（Chinese, English）
- [x] 定义 `TranslationKey` 枚举（包含所有翻译键）
- [x] 实现 `I18n` 结构体
- [x] 实现 `I18n::new()` 和翻译初始化
- [x] 实现 `I18n::t()` 翻译方法
- [x] 实现 `I18n::set_language()` 语言切换
- [x] 实现语言检测功能（从环境变量或配置）
- [x] 添加所有界面文本的中英文翻译

**Acceptance Criteria**:
- `I18n::new(Language::Chinese)` 可以创建中文翻译器
- `i18n.t(TranslationKey::MainMenu)` 返回正确翻译
- 切换语言后翻译正确更新
- 所有界面文本都有对应的翻译键

---

## Phase 2: 主菜单界面

### Task 2.1: 主菜单结构
- [x] 定义 `MainMenuScreen` struct
- [x] 定义菜单项
- [x] 实现 `new()`

**Acceptance Criteria**: 主菜单可以创建

---

### Task 2.2: 主菜单渲染
- [x] 实现 `render()`，接收 `i18n` 参数
- [x] 使用 `i18n.t()` 获取所有文本翻译
- [x] 显示标题（使用翻译）
- [x] 显示球队信息（使用翻译）
- [x] 显示菜单列表（使用翻译）

**Acceptance Criteria**: 主菜单正确显示，支持中英文切换

---

### Task 2.3: 主菜单交互
- [x] 实现 `handle_key()`
- [x] 处理上下键选择
- [x] 处理回车确认
- [x] 处理退出

**Acceptance Criteria**: 可以用键盘导航菜单

---

## Phase 3: 球队管理界面

### Task 3.1: 球队管理结构
- [x] 定义 `TeamManagementScreen` struct
- [x] 定义标签页枚举

**Acceptance Criteria**: 结构定义完成

---

### Task 3.2: 阵容标签页
- [x] 实现阵容列表渲染
- [x] 显示球员表格
- [x] 实现选择功能

**Acceptance Criteria**: 能显示球员列表

---

### Task 3.3: 战术标签页
- [x] 实现战术信息显示
- [x] 显示当前阵型
- [x] 显示球员角色

**Acceptance Criteria**: 能显示战术信息

---

### Task 3.4: 统计标签页
- [x] 实现统计数据展示
- [x] 显示球队数据

**Acceptance Criteria**: 能显示统计数据

---

### Task 3.5: 球队管理交互
- [x] 实现标签切换
- [x] 实现球员选择
- [x] 实现返回功能

**Acceptance Criteria**: 可以在标签间切换

---

## Phase 4: 战术设置界面

### Task 4.1: 战术界面结构
- [x] 定义 `TacticsScreen` struct
- [x] 定义选择区域枚举

**Acceptance Criteria**: 结构定义完成

---

### Task 4.2: 阵型选择
- [x] 显示可选阵型
- [x] 实现阵型切换
- [x] 显示阵型图

**Acceptance Criteria**: 可以选择阵型

---

### Task 4.3: 球员角色设置
- [x] 显示每个位置的角色
- [x] 实现角色修改
- [x] 实现职责修改

**Acceptance Criteria**: 可以修改球员角色

---

### Task 4.4: 团队指令
- [x] 显示进攻倾向滑块
- [x] 显示防守高度选择
- [x] 显示传球风格选择
- [x] 显示比赛节奏选择

**Acceptance Criteria**: 可以调整团队指令

---

## Phase 5: 转会市场界面

### Task 5.1: 转会市场结构
- [x] 定义 `TransferMarketScreen` struct
- [x] 定义标签页

**Acceptance Criteria**: 结构定义完成

---

### Task 5.2: 浏览标签页
- [x] 显示市场球员列表
- [x] 显示筛选选项
- [x] 实现球员选择

**Acceptance Criteria**: 能浏览转会市场

---

### Task 5.3: 购买功能
- [x] 显示球员详情
- [x] 显示价格
- [ ] 实现确认购买

**Status**: ⚠️ 未完全实现 - 代码中有TODO标记

**Acceptance Criteria**: 可以购买球员

---

### Task 5.4: 出售标签页
- [x] 显示本队球员
- [ ] 实现挂牌功能
- [ ] 设置售价

**Status**: ⚠️ 未完全实现 - 转会功能有待完善

**Acceptance Criteria**: 可以挂牌出售球员

---

## Phase 6: 比赛相关界面

### Task 6.1: 模式选择界面
- [x] 定义 `MatchModeSelectionScreen`
- [x] 显示两种模式选项
- [x] 实现模式选择

**Acceptance Criteria**: 可以选择比赛模式

---

### Task 6.2: 文本直播界面
- [x] 定义 `MatchLiveScreen`
- [x] 实时显示比分
- [x] 显示比赛时间
- [x] 显示事件列表
- [ ] 实现暂停/继续

**Status**: ⚠️ 未完全实现 - 比赛控制功能有待完善

**Acceptance Criteria**: 能实时显示比赛进程

---

### Task 6.3: 中场休息
- [x] 显示中场界面
- [x] 允许换人
- [x] 允许调整战术
- [x] 更衣室发言选项

**Acceptance Criteria**: 中场可以调整

---

### Task 6.4: 比赛结果界面
- [x] 显示最终比分
- [x] 显示统计数据
- [x] 显示关键事件
- [x] 按任意键继续

**Acceptance Criteria**: 能显示比赛结果

---

### Task 6.5: 快速模拟显示
- [x] 实现简化版比赛界面
- [x] 显示模拟进度
- [x] 显示最终结果

**Acceptance Criteria**: 快速模拟可以完成

---

## Phase 7: 联赛积分榜

### Task 7.1: 积分榜界面
- [x] 定义 `LeagueTableScreen`
- [x] 显示积分榜表格
- [x] 排序（积分、净胜球）
- [x] 高亮玩家球队

**Acceptance Criteria**: 能正确显示积分榜

---

## Phase 8: 存档界面

### Task 8.1: 存档/读取界面
- [x] 定义 `SaveLoadScreen`
- [x] 显示存档列表
- [ ] 实现保存功能
- [ ] 实现读取功能

**Status**: ⚠️ 未完全实现 - 代码中有TODO标记（save/load logic）

**Acceptance Criteria**: 可以存档和读档

---

## Phase 9: 可复用组件

### Task 9.1: 球员信息卡片
- [x] 实现 `render_player_card()`
- [x] 显示关键信息

---

### Task 9.2: 进度条
- [x] 实现 `render_progress_bar()`
- [x] 支持不同颜色

---

### Task 9.3: 确认对话框
- [x] 实现 `render_confirmation_dialog()`
- [x] 返回用户选择

---

### Task 9.4: 消息提示
- [x] 实现错误消息显示
- [x] 实现成功消息显示
- [x] 实现信息消息显示

---

## Phase 10: 样式和主题

### Task 10.1: 颜色方案
- [x] 定义 `Theme` struct
- [x] 实现默认主题
- [x] 实现颜色使用

---

### Task 10.2: 布局优化
- [x] 优化各个界面布局
- [x] 确保不同终端尺寸兼容

---

## Phase 11: 输入处理

### Task 11.1: 事件处理
- [x] 实现 `read()` 函数
- [x] 处理键盘事件
- [x] 处理鼠标事件（可选）

---

### Task 11.2: 快捷键
- [x] 定义全局快捷键（q退出，Esc返回等）
- [x] 实现上下文快捷键

---

## Phase 12: 模块集成

### Task 12.1: 连接游戏状态
- [x] UI连接到GameState
- [x] 实现状态更新后重绘

---

### Task 12.2: 错误处理
- [x] 捕获并显示错误
- [x] 优雅处理异常

---

### Task 12.3: 性能优化
- [x] 优化重绘频率
- [x] 减少不必要的渲染

---

## Phase 13: 测试

### Task 13.1: 单元测试
- [x] 测试各个Screen的渲染
- [x] 测试事件处理

---

### Task 13.2: 集成测试
- [x] 测试完整用户流程
- [x] 测试界面切换

---

### Task 13.3: 用户体验测试
- [x] 测试不同终端尺寸
- [x] 测试边界情况

---

## 依赖关系

```
Phase 1 (框架) → Phase 2 (主菜单) → Phase 3 (球队) → Phase 4 (战术)
                                              ↓
                                         Phase 5 (转会)
                                              ↓
                                         Phase 6 (比赛)
                                              ↓
                                         Phase 7 (积分榜)
                                              ↓
                                         Phase 8 (存档)
                                              ↓
                                         Phase 9 (组件)
                                              ↓
                                        Phase 10 (样式)
                                              ↓
                                        Phase 11 (输入)
                                              ↓
                                        Phase 12 (集成)
                                              ↓
                                        Phase 13 (测试)
```

---

## 预估时间

- Phase 1: 2天
- Phase 2: 2-3天
- Phase 3: 3-4天
- Phase 4: 3-4天
- Phase 5: 3-4天
- Phase 6: 5-7天（文本直播比较复杂）
- Phase 7: 1-2天
- Phase 8: 2天
- Phase 9: 2天
- Phase 10: 1-2天
- Phase 11: 1-2天
- Phase 12: 2-3天
- Phase 13: 2-3天

**总计**: 约 35-45 天

---

## 注意事项

1. **响应式设计**: 考虑不同终端尺寸（80x24 最小，推荐 120x40）
2. **性能**: 避免频繁重绘，只在状态变化时重绘
3. **可访问性**: 使用清晰的布局和颜色
4. **键盘导航**: 确保所有功能都可以用键盘操作
5. **错误显示**: 错误信息应该清晰易懂
6. **一致性**: 所有界面保持一致的设计风格
7. **国际化**: 已实现中英文双语支持，所有界面文本使用翻译键
8. **测试**: 在真实终端中测试，不仅仅是CI环境
9. **语言切换**: 确保所有界面在切换语言后能正确更新显示
