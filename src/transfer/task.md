# Transfer Module Implementation Tasks

## 实际完成情况摘要

**最后更新日期**: 2026-02-01

**真实完成度**: 约 80% (59/73 tasks)

**主要缺失功能**:
- Phase 5: 合同管理（renew_contract，检查合同到期）- 未实现
- Phase 7: 辅助功能（evaluate_player_value, predict_potential_value, 转会历史）- 部分未实现

**已完成模块**:
- Phase 1-4: 基础结构、浏览、购买、出售功能 - ✅ 完整实现
- Phase 6: AI转会决策（transfer_decision.rs in ai module）- ✅ 完整实现

---

## Phase 1: 基础结构

### Task 1.1: 错误类型定义
- [x] 定义 `TransferError` enum
- [x] 实现所有错误变体
- [x] 添加 `thiserror` derive

**Acceptance Criteria**: 错误类型完整，可以正常使用

---

### Task 1.2: 数据结构定义
- [x] 定义 `TransferMarket` struct
- [x] 定义 `TransferOffer` struct
- [x] 定义 `OfferStatus` enum
- [x] 定义 `MarketFilter` struct

**Acceptance Criteria**: 所有结构定义完成

---

### Task 1.3: TransferMarket 构造函数
- [x] 实现 `TransferMarket::new()`
- [x] 接受 Repository 依赖注入

**Acceptance Criteria**:
```rust
let market = TransferMarket::new(player_repo, team_repo, transfer_repo);
```

---

## Phase 2: 浏览功能

### Task 2.1: 获取市场球员
- [x] 实现 `get_market_players()`
- [x] 调用 `TransferMarketRepository`

**Acceptance Criteria**: 返回转会市场上的所有球员

---

### Task 2.2: 筛选功能
- [x] 实现 `search_players()`
- [x] 实现位置筛选
- [x] 实现能力范围筛选
- [x] 实现年龄筛选
- [x] 实现薪资筛选
- [x] 实现价格筛选

**Acceptance Criteria**:
```rust
let filters = MarketFilter {
    positions: Some(vec![Position::ST]),
    max_age: Some(25),
    ..Default::default()
};
let results = market.search_players(filters).await?;
assert!(results.iter().all(|p| p.position == Position::ST));
```

---

## Phase 3: 购买功能

### Task 3.1: 基础购买逻辑
- [x] 实现 `buy_player()`
- [x] 获取球队和球员
- [x] 检查预算
- [x] 扣除预算
- [x] 更新球员所属球队
- [x] 从转会市场移除
- [x] 持久化变更

**Acceptance Criteria**:
```rust
market.buy_player("team1", "player1", 1000000).await?;
// 球员应该属于新球队
// 预算应该扣除
// 不再在转会市场
```

---

### Task 3.2: 预算检查
- [x] 实现预算验证
- [x] 返回清晰的错误信息

**Acceptance Criteria**:
```rust
let result = market.buy_player("team1", "player1", 999999999);
assert!(matches!(result, Err(TransferError::InsufficientFunds { .. })));
```

---

### Task 3.3: 合同生成
- [x] 实现新球员合同生成
- [x] 计算新薪资
- [x] 设置合同年限

**Acceptance Criteria**: 购买的球员有合理的合同

---

## Phase 4: 出售功能

### Task 4.1: 挂牌球员
- [x] 实现 `list_player()`
- [x] 验证所有权
- [x] 验证价格合理性
- [x] 添加到转会市场

**Acceptance Criteria**:
```rust
market.list_player("team1", "player1", 5000000).await?;
// 球员应该在转会市场
```

---

### Task 4.2: 价格验证
- [x] 实现价格范围检查（50%-150% market_value）
- [x] 返回错误如果价格不合理

**Acceptance Criteria**:
```rust
// 价格太低
let result = market.list_player("team1", "player1", 100);
assert!(matches!(result, Err(TransferError::UnreasonablePrice)));
```

---

### Task 4.3: 取消挂牌
- [x] 实现 `delist_player()`
- [x] 验证所有权
- [x] 从转会市场移除

**Acceptance Criteria**: 球员不再在转会市场

---

## Phase 5: 合同管理

### Task 5.1: 续约功能
- [ ] 实现 `renew_contract()`
- [ ] 验证所有权
- [ ] 更新薪资和年限
- [ ] 持久化

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**: 续约成功

---

### Task 5.2: 合同到期处理
- [ ] 实现检查合同到期
- [ ] 返回即将到期球员列表
- [ ] 可选：自动续约提示

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

**Acceptance Criteria**: 能识别合同即将到期的球员

---

## Phase 6: AI转会决策

### Task 6.1: 弱点评估
- [x] 实现 `evaluate_weaknesses()`
- [x] 分析球队各位置实力
- [x] 返回需要补强的位置

**Status**: ✅ Implemented in src/ai/transfer_decision.rs

**Acceptance Criteria**:
```rust
let weaknesses = market.evaluate_weaknesses(&team).await?;
// 返回需要补强的位置列表
```

---

### Task 6.2: AI购买决策
- [x] 实现 `decide_ai_transfer()`
- [x] 评估球队需求
- [x] 设定预算
- [x] 寻找合适球员
- [x] 生成报价或返回None

**Status**: ✅ Implemented in src/ai/transfer_decision.rs

**Acceptance Criteria**: AI能做出合理的转会决策

---

### Task 6.3: 批量处理AI转会
- [x] 实现 `process_ai_transfers()`
- [x] 遍历所有AI球队
- [x] 收集所有转会报价

**Status**: ✅ Implemented in src/ai/transfer_decision.rs

**Acceptance Criteria**: 能为所有AI球队生成转会决策

---

## Phase 7: 辅助功能

### Task 7.1: 球员估值
- [ ] 实现 `evaluate_player_value()`
- [ ] 返回市场价值

**Status**: ⚠️ 未实现 - 代码中未找到对应实现（Player已有market_value字段，但独立估值函数未实现）

---

### Task 7.2: 潜力预测
- [ ] 实现 `predict_potential_value()`
- [ ] 考虑年龄和潜力

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

---

### Task 7.3: 转会历史
- [ ] 实现记录转会历史
- [ ] 实现查询转会历史
- [ ] 显示最近转会

**Status**: ⚠️ 未实现 - 代码中未找到对应实现

---

## Phase 8: 模块导出

### Task 8.1: mod.rs
- [x] 导出公共类型
- [x] 导出错误类型

**Acceptance Criteria**: 其他模块可以正常使用

---

## Phase 9: 测试

### Task 9.1: 单元测试
- [x] 测试购买流程
- [x] 测试出售流程
- [x] 测试筛选功能
- [x] 测试预算检查
- [x] 测试价格验证
- [x] 测试AI决策

**Acceptance Criteria**: `cargo test --lib transfer` 全部通过

---

### Task 9.2: 集成测试
- [x] 测试完整转会流程
  - 挂牌 → AI购买 → 数据更新
- [x] 测试多个AI球队同时转会

---

## 更新记录

### 2026-02-01 状态更新
- ✅ Phase 6 (AI转会决策) 标记为完成 - 实现在 src/ai/transfer_decision.rs
  - evaluate_weaknesses() ✅
  - decide_ai_transfer() ✅
  - process_ai_transfers() ✅
- 更新完成度：65% → 80% (59/73 tasks)

---

## 依赖关系

```
Phase 1 → Phase 2 (浏览) → Phase 3 (购买)
              ↓              ↓
         Phase 4 (出售) ←────┘
              ↓
         Phase 5 (合同)
              ↓
         Phase 6 (AI决策)
              ↓
         Phase 7 (辅助)
              ↓
         Phase 8 (导出)
              ↓
         Phase 9 (测试)
```

---

## 预估时间

- Phase 1: 1天
- Phase 2: 1-2天
- Phase 3: 2-3天
- Phase 4: 2天
- Phase 5: 1天
- Phase 6: 3-4天
- Phase 7: 1天
- Phase 8: 0.5天
- Phase 9: 2-3天

**总计**: 约 15-20 天

---

## 注意事项

1. **事务支持**: 转会涉及多个数据库操作，考虑使用事务
2. **并发安全**: 如果支持多线程，需要注意并发问题
3. **价格合理性**: 防止玩家用极低价格购买球员
4. **AI平衡**: AI转会不能太激进或太保守
5. **薪资上限**: 可选：实现薪资上限规则
6. **合同谈判**: 未来可以扩展为更复杂的谈判系统
