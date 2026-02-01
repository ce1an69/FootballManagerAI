# Transfer Module Design

## 概述

Transfer模块负责管理游戏中的转会市场，包括球员购买、出售、市场浏览等功能。

## 架构

### 文件结构

```
transfer/
├── mod.rs          # 模块导出
└── market.rs       # 转会市场核心逻辑
```

## 核心功能

### 1. 转会市场 (market.rs)

#### 1.1 数据结构

```rust
use crate::data::repository::{PlayerRepository, TeamRepository, TransferMarketRepository};
use crate::team::models::{Player, Team};
use crate::error::{GameError, TransferError};

pub struct TransferMarket {
    player_repo: Box<dyn PlayerRepository>,
    team_repo: Box<dyn TeamRepository>,
    transfer_repo: Box<dyn TransferMarketRepository>,
}

/// 转会窗口类型
#[derive(Debug, Clone, PartialEq)]
pub enum TransferWindow {
    Summer,   // 夏季转会窗口：6月1日 - 8月31日
    Winter,   // 冬季转会窗口：1月1日 - 1月31日
    Closed,   // 转会窗口关闭
}

impl TransferWindow {
    /// 根据游戏日期判断当前转会窗口状态
    pub fn from_date(date: &crate::game::GameDate) -> Self {
        if date.is_summer_transfer_window() {
            TransferWindow::Summer
        } else if date.is_winter_transfer_window() {
            TransferWindow::Winter
        } else {
            TransferWindow::Closed
        }
    }

    /// 是否可以进行转会
    pub fn is_open(&self) -> bool {
        !matches!(self, TransferWindow::Closed)
    }
}

#[derive(Debug, Clone)]
pub struct TransferOffer {
    pub player_id: String,
    pub from_team_id: String,
    pub to_team_id: String,
    pub offer_amount: u32,
    pub wage_offer: u32,
    pub contract_years: u8,
    pub status: OfferStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OfferStatus {
    Pending,
    Accepted,
    Rejected,
    Withdrawn,
}

#[derive(Debug, Clone)]
pub struct MarketFilter {
    pub positions: Option<Vec<crate::team::Position>>,
    pub min_ability: Option<u16>,
    pub max_ability: Option<u16>,
    pub max_age: Option<u8>,
    pub max_wage: Option<u32>,
    pub max_price: Option<u32>,
}
```

#### 1.2 核心方法

##### 浏览转会市场

```rust
impl TransferMarket {
    /// 获取转会市场上的所有球员
    pub fn get_market_players(&self) -> Result<Vec<Player>, GameError> {
        self.transfer_repo.get_market_players()
            .map_err(GameError::from)
    }

    /// 根据筛选条件搜索球员
    pub fn search_players(
        &self,
        filters: MarketFilter,
    ) -> Result<Vec<Player>, GameError> {
        let mut players = self.get_market_players()?;

        if let Some(positions) = filters.positions {
            players.retain(|p| positions.contains(&p.position));
        }

        if let Some(min_ability) = filters.min_ability {
            players.retain(|p| p.current_ability >= min_ability);
        }

        if let Some(max_ability) = filters.max_ability {
            players.retain(|p| p.current_ability <= max_ability);
        }

        if let Some(max_age) = filters.max_age {
            players.retain(|p| p.age <= max_age);
        }

        if let Some(max_wage) = filters.max_wage {
            players.retain(|p| p.wage <= max_wage);
        }

        if let Some(max_price) = filters.max_price {
            players.retain(|p| p.market_value <= max_price);
        }

        Ok(players)
    }
}
```

##### 购买球员

```rust
impl TransferMarket {
    /// 玩家购买球员
    pub fn buy_player(
        &self,
        buyer_team_id: &str,
        player_id: &str,
        offer_amount: u32,
        current_date: &crate::game::GameDate,
    ) -> Result<(), TransferError> {
        // 0. 检查转会窗口是否开放
        let window = TransferWindow::from_date(current_date);
        if !window.is_open() {
            return Err(TransferError::TransferWindowClosed);
        }

        // 1. 获取球队和球员
        let mut team = self.team_repo.get_by_id(buyer_team_id)?;
        let player = self.player_repo.get_by_id(player_id)?;

        // 2. 检查球员是否在市场上
        let market_listing = self.transfer_repo.get_market_listing(player_id)?;
        if market_listing.is_none() {
            return Err(TransferError::PlayerNotOnMarket);
        }

        // 3. 检查预算
        if team.budget < offer_amount {
            return Err(TransferError::InsufficientFunds {
                required: offer_amount,
                available: team.budget,
            });
        }

        // 4. 检查薪资空间（可选：薪资上限规则）
        // 5. 执行转会
        team.budget -= offer_amount;
        let mut updated_player = player.clone();
        updated_player.team_id = Some(buyer_team_id.to_string());

        // 6. 更新合同
        updated_player.wage = calculate_wage(&updated_player);
        updated_player.contract_years = 3;  // 默认3年

        // 7. 持久化
        self.team_repo.update(&team)?;
        self.player_repo.update(&updated_player)?;
        self.transfer_repo.remove_from_market(player_id)?;

        Ok(())
    }
}
```

##### 出售球员

```rust
impl TransferMarket {
    /// 将球员挂牌出售
    pub fn list_player(
        &self,
        seller_team_id: &str,
        player_id: &str,
        asking_price: u32,
    ) -> Result<(), TransferError> {
        // 1. 验证球员属于该球队
        let player = self.player_repo.get_by_id(player_id)?;
        if player.team_id.as_ref() != Some(&seller_team_id.to_string()) {
            return Err(TransferError::PlayerNotOwned);
        }

        // 2. 验证售价合理（不能太高或太低）
        let reasonable_min = (player.market_value as f32 * 0.5) as u32;
        let reasonable_max = (player.market_value as f32 * 1.5) as u32;
        if asking_price < reasonable_min || asking_price > reasonable_max {
            return Err(TransferError::UnreasonablePrice);
        }

        // 3. 添加到转会市场
        self.transfer_repo.add_to_market(player_id, asking_price)?;

        Ok(())
    }

    /// AI球队决定是否购买
    pub fn process_ai_transfers(
        &self,
        teams: &[Team],
        player_team_id: &str,
    ) -> Result<Vec<TransferOffer>, GameError> {
        let mut offers = vec![];

        for team in teams {
            // 跳过玩家球队
            if team.id == player_team_id {
                continue;
            }

            // AI决策逻辑
            if let Some(offer) = self.decide_ai_transfer(team)? {
                offers.push(offer);
            }
        }

        Ok(offers)
    }

    fn decide_ai_transfer(
        &self,
        team: &Team,
    ) -> Result<Option<TransferOffer>, GameError> {
        // 1. 评估球队弱点
        let weak_positions = self.evaluate_weaknesses(team)?;

        // 2. 设定预算（基于财政状况）
        let transfer_budget = (team.budget as f32 * 0.3) as u32;

        // 3. 寻找合适的球员
        let market_players = self.get_market_players()?;
        let candidates: Vec<_> = market_players
            .into_iter()
            .filter(|p| {
                weak_positions.contains(&p.position)
                    && p.market_value <= transfer_budget
            })
            .collect();

        if candidates.is_empty() {
            return Ok(None);
        }

        // 4. 选择最佳候选
        let best_player = candidates
            .into_iter()
            .max_by_key(|p| p.current_ability)
            .unwrap();

        // 5. 生成报价
        Ok(Some(TransferOffer {
            player_id: best_player.id.clone(),
            from_team_id: team.id.clone(),
            to_team_id: best_player.team_id.clone().unwrap_or_default(),
            offer_amount: best_player.market_value,
            wage_offer: best_player.wage,
            contract_years: 3,
            status: OfferStatus::Pending,
        }))
    }

    fn evaluate_weaknesses(&self, team: &Team) -> Result<Vec<crate::team::Position>, GameError> {
        // 简单实现：检查每个位置的球员数量和能力
        // 返回需要补强的位置列表
        let players = self.player_repo.get_by_team(&team.id)?;
        let mut weak_positions = vec![];

        // 检查各位置球员数量
        let position_counts: std::collections::HashMap<_, usize> = players
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, p| {
                *acc.entry(p.position.clone()).or_insert(0) += 1;
                acc
            });

        // 如果某位置球员少于2人，视为弱点
        use crate::team::Position;
        let required_positions = vec![
            Position::GK, Position::CB, Position::LB, Position::RB,
            Position::CM, Position::ST,
        ];

        for pos in required_positions {
            if position_counts.get(&pos).unwrap_or(&0) < &2 {
                weak_positions.push(pos);
            }
        }

        Ok(weak_positions)
    }
}
```

##### 取消挂牌

```rust
impl TransferMarket {
    /// 从转会市场移除球员
    pub fn delist_player(
        &self,
        seller_team_id: &str,
        player_id: &str,
    ) -> Result<(), TransferError> {
        // 验证所有权
        let player = self.player_repo.get_by_id(player_id)?;
        if player.team_id.as_ref() != Some(&seller_team_id.to_string()) {
            return Err(TransferError::PlayerNotOwned);
        }

        self.transfer_repo.remove_from_market(player_id)?;
        Ok(())
    }
}
```

### 2. 转会评估

```rust
impl TransferMarket {
    /// 评估球员的市场价值
    pub fn evaluate_player_value(&self, player: &Player) -> u32 {
        player.market_value
    }

    /// 预测球员的潜力价值
    pub fn predict_potential_value(&self, player: &Player) -> u32 {
        if player.age <= 21 {
            // 青年球员有潜力溢价
            player.market_value + (player.potential_ability - player.current_ability) as u32 * 1000
        } else {
            player.market_value
        }
    }
}
```

### 3. 合同管理

```rust
impl TransferMarket {
    /// 续约合同
    pub fn renew_contract(
        &self,
        team_id: &str,
        player_id: &str,
        new_wage: u32,
        new_years: u8,
    ) -> Result<(), TransferError> {
        let mut player = self.player_repo.get_by_id(player_id)?;

        // 验证所有权
        if player.team_id.as_ref() != Some(&team_id.to_string()) {
            return Err(TransferError::PlayerNotOwned);
        }

        // 续约
        player.wage = new_wage;
        player.contract_years = new_years;

        self.player_repo.update(&player)?;
        Ok(())
    }
}
```

## 错误类型

```rust
#[derive(Debug, thiserror::Error)]
pub enum TransferError {
    #[error("Player not found")]
    PlayerNotFound,

    #[error("Team not found")]
    TeamNotFound,

    #[error("Player not on transfer market")]
    PlayerNotOnMarket,

    #[error("Player not owned by this team")]
    PlayerNotOwned,

    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds { required: u32, available: u32 },

    #[error("Unreasonable asking price")]
    UnreasonablePrice,

    #[error("Transfer rejected by player")]
    RejectedByPlayer,

    #[error("Contract negotiation failed")]
    ContractNegotiationFailed,

    #[error("Transfer window is closed")]
    TransferWindowClosed,
}
```

## UI交互流程

### 购买流程

```
1. 玩家打开"转会市场"
   ↓
2. 显示可用球员列表（可筛选）
   ↓
3. 玩家选择球员查看详情
   ↓
4. 显示球员信息 + 询问价格 + 球队预算
   ↓
5. 玩家确认购买
   ↓
6. 检查预算 → 执行转会 → 更新数据库
   ↓
7. 显示成功消息或错误
```

### 出售流程

```
1. 玩家选择"出售球员"
   ↓
2. 显示本队球员列表
   ↓
3. 玩家选择要出售的球员
   ↓
4. 显示建议价格范围（基于market_value）
   ↓
5. 玩家输入要价
   ↓
6. 系统验证价格合理性
   ↓
7. 挂牌到转会市场
   ↓
8. 等待AI或其他球队购买
```

## AI决策

### AI球队转会优先级

1. **评估弱点**: 检查哪些位置需要补强
2. **设定预算**: 基于当前财政状况
3. **寻找目标**: 在转会市场寻找合适球员
4. **生成报价**: 如果找到合适球员，发起报价
5. **决策**: 基于球队风格和需求

### 球队风格影响

- **Attacking**: 优先购买前锋、攻击型中场
- **Defending**: 优先购买中后卫、防守型中场
- **YouthDevelopment**: 优先购买21岁以下潜力球员
- **Balanced**: 平衡各个位置

## 依赖

- `data` 模块：数据访问
- `team` 模块：数据模型
- `ai` 模块：AI决策逻辑（可选）

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buy_player_success() {
        // 设置测试数据
        // 执行购买
        // 验证结果
    }

    #[test]
    fn test_buy_player_insufficient_funds() {
        // 测试预算不足场景
    }

    #[test]
    fn test_search_players_filtering() {
        // 测试筛选功能
    }
}
```
