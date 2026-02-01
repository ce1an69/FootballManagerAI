# UI Module Implementation Tasks

## 重要说明

**国际化要求**: 所有界面的 `render()` 方法必须接收 `i18n: &I18n` 参数，并使用 `i18n.t(TranslationKey::...)` 获取翻译文本。禁止在界面代码中硬编码中文字符串。

## Phase 1: 基础框架

### Task 1.1: 项目依赖
- [ ] 添加 `ratatui` 依赖
- [ ] 添加 `crossterm` 依赖
- [ ] 配置 Cargo.toml

**Acceptance Criteria**: `cargo build` 成功

---

### Task 1.2: App 结构
- [ ] 定义 `App` struct
- [ ] 在 `App` 中添加 `i18n: I18n` 字段
- [ ] 实现 `App::new()`，支持语言参数
- [ ] 实现语言检测逻辑
- [ ] 实现 `run()` 方法框架
- [ ] 更新 `render()` 方法，传递 `i18n` 给 Screen

**Acceptance Criteria**: App 可以创建和运行，包含国际化支持

---

### Task 1.3: Screen Trait
- [ ] 定义 `Screen` trait
- [ ] 定义所有必需方法

**Acceptance Criteria**: Trait 定义完整

---

### Task 1.4: 国际化支持
- [ ] 创建 `i18n.rs` 文件
- [ ] 定义 `Language` 枚举（Chinese, English）
- [ ] 定义 `TranslationKey` 枚举（包含所有翻译键）
- [ ] 实现 `I18n` 结构体
- [ ] 实现 `I18n::new()` 和翻译初始化
- [ ] 实现 `I18n::t()` 翻译方法
- [ ] 实现 `I18n::set_language()` 语言切换
- [ ] 实现语言检测功能（从环境变量或配置）
- [ ] 添加所有界面文本的中英文翻译

**Acceptance Criteria**:
- `I18n::new(Language::Chinese)` 可以创建中文翻译器
- `i18n.t(TranslationKey::MainMenu)` 返回正确翻译
- 切换语言后翻译正确更新
- 所有界面文本都有对应的翻译键

---

## Phase 2: 主菜单界面

### Task 2.1: 主菜单结构
- [ ] 定义 `MainMenuScreen` struct
- [ ] 定义菜单项
- [ ] 实现 `new()`

**Acceptance Criteria**: 主菜单可以创建

---

### Task 2.2: 主菜单渲染
- [ ] 实现 `render()`，接收 `i18n` 参数
- [ ] 使用 `i18n.t()` 获取所有文本翻译
- [ ] 显示标题（使用翻译）
- [ ] 显示球队信息（使用翻译）
- [ ] 显示菜单列表（使用翻译）

**Acceptance Criteria**: 主菜单正确显示，支持中英文切换

---

### Task 2.3: 主菜单交互
- [ ] 实现 `handle_key()`
- [ ] 处理上下键选择
- [ ] 处理回车确认
- [ ] 处理退出

**Acceptance Criteria**: 可以用键盘导航菜单

---

## Phase 3: 球队管理界面

### Task 3.1: 球队管理结构
- [ ] 定义 `TeamManagementScreen` struct
- [ ] 定义标签页枚举

**Acceptance Criteria**: 结构定义完成

---

### Task 3.2: 阵容标签页
- [ ] 实现阵容列表渲染
- [ ] 显示球员表格
- [ ] 实现选择功能

**Acceptance Criteria**: 能显示球员列表

---

### Task 3.3: 战术标签页
- [ ] 实现战术信息显示
- [ ] 显示当前阵型
- [ ] 显示球员角色

**Acceptance Criteria**: 能显示战术信息

---

### Task 3.4: 统计标签页
- [ ] 实现统计数据展示
- [ ] 显示球队数据

**Acceptance Criteria**: 能显示统计数据

---

### Task 3.5: 球队管理交互
- [ ] 实现标签切换
- [ ] 实现球员选择
- [ ] 实现返回功能

**Acceptance Criteria**: 可以在标签间切换

---

## Phase 4: 战术设置界面

### Task 4.1: 战术界面结构
- [ ] 定义 `TacticsScreen` struct
- [ ] 定义选择区域枚举

**Acceptance Criteria**: 结构定义完成

---

### Task 4.2: 阵型选择
- [ ] 显示可选阵型
- [ ] 实现阵型切换
- [ ] 显示阵型图

**Acceptance Criteria**: 可以选择阵型

---

### Task 4.3: 球员角色设置
- [ ] 显示每个位置的角色
- [ ] 实现角色修改
- [ ] 实现职责修改

**Acceptance Criteria**: 可以修改球员角色

---

### Task 4.4: 团队指令
- [ ] 显示进攻倾向滑块
- [ ] 显示防守高度选择
- [ ] 显示传球风格选择
- [ ] 显示比赛节奏选择

**Acceptance Criteria**: 可以调整团队指令

---

## Phase 5: 转会市场界面

### Task 5.1: 转会市场结构
- [ ] 定义 `TransferMarketScreen` struct
- [ ] 定义标签页

**Acceptance Criteria**: 结构定义完成

---

### Task 5.2: 浏览标签页
- [ ] 显示市场球员列表
- [ ] 显示筛选选项
- [ ] 实现球员选择

**Acceptance Criteria**: 能浏览转会市场

---

### Task 5.3: 购买功能
- [ ] 显示球员详情
- [ ] 显示价格
- [ ] 实现确认购买

**Acceptance Criteria**: 可以购买球员

---

### Task 5.4: 出售标签页
- [ ] 显示本队球员
- [ ] 实现挂牌功能
- [ ] 设置售价

**Acceptance Criteria**: 可以挂牌出售球员

---

## Phase 6: 比赛相关界面

### Task 6.1: 模式选择界面
- [ ] 定义 `MatchModeSelectionScreen`
- [ ] 显示两种模式选项
- [ ] 实现模式选择

**Acceptance Criteria**: 可以选择比赛模式

---

### Task 6.2: 文本直播界面
- [ ] 定义 `MatchLiveScreen`
- [ ] 实时显示比分
- [ ] 显示比赛时间
- [ ] 显示事件列表
- [ ] 实现暂停/继续

**Acceptance Criteria**: 能实时显示比赛进程

---

### Task 6.3: 中场休息
- [ ] 显示中场界面
- [ ] 允许换人
- [ ] 允许调整战术
- [ ] 更衣室发言选项

**Acceptance Criteria**: 中场可以调整

---

### Task 6.4: 比赛结果界面
- [ ] 显示最终比分
- [ ] 显示统计数据
- [ ] 显示关键事件
- [ ] 按任意键继续

**Acceptance Criteria**: 能显示比赛结果

---

### Task 6.5: 快速模拟显示
- [ ] 实现简化版比赛界面
- [ ] 显示模拟进度
- [ ] 显示最终结果

**Acceptance Criteria**: 快速模拟可以完成

---

## Phase 7: 联赛积分榜

### Task 7.1: 积分榜界面
- [ ] 定义 `LeagueTableScreen`
- [ ] 显示积分榜表格
- [ ] 排序（积分、净胜球）
- [ ] 高亮玩家球队

**Acceptance Criteria**: 能正确显示积分榜

---

## Phase 8: 存档界面

### Task 8.1: 存档/读取界面
- [ ] 定义 `SaveLoadScreen`
- [ ] 显示存档列表
- [ ] 实现保存功能
- [ ] 实现读取功能

**Acceptance Criteria**: 可以存档和读档

---

## Phase 9: 可复用组件

### Task 9.1: 球员信息卡片
- [ ] 实现 `render_player_card()`
- [ ] 显示关键信息

---

### Task 9.2: 进度条
- [ ] 实现 `render_progress_bar()`
- [ ] 支持不同颜色

---

### Task 9.3: 确认对话框
- [ ] 实现 `render_confirmation_dialog()`
- [ ] 返回用户选择

---

### Task 9.4: 消息提示
- [ ] 实现错误消息显示
- [ ] 实现成功消息显示
- [ ] 实现信息消息显示

---

## Phase 10: 样式和主题

### Task 10.1: 颜色方案
- [ ] 定义 `Theme` struct
- [ ] 实现默认主题
- [ ] 实现颜色使用

---

### Task 10.2: 布局优化
- [ ] 优化各个界面布局
- [ ] 确保不同终端尺寸兼容

---

## Phase 11: 输入处理

### Task 11.1: 事件处理
- [ ] 实现 `read()` 函数
- [ ] 处理键盘事件
- [ ] 处理鼠标事件（可选）

---

### Task 11.2: 快捷键
- [ ] 定义全局快捷键（q退出，Esc返回等）
- [ ] 实现上下文快捷键

---

## Phase 12: 模块集成

### Task 12.1: 连接游戏状态
- [ ] UI连接到GameState
- [ ] 实现状态更新后重绘

---

### Task 12.2: 错误处理
- [ ] 捕获并显示错误
- [ ] 优雅处理异常

---

### Task 12.3: 性能优化
- [ ] 优化重绘频率
- [ ] 减少不必要的渲染

---

## Phase 13: 测试

### Task 13.1: 单元测试
- [ ] 测试各个Screen的渲染
- [ ] 测试事件处理

---

### Task 13.2: 集成测试
- [ ] 测试完整用户流程
- [ ] 测试界面切换

---

### Task 13.3: 用户体验测试
- [ ] 测试不同终端尺寸
- [ ] 测试边界情况

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
