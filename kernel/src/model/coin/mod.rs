use chrono::{DateTime, Duration, Utc};
use std::str::FromStr;

use super::id::{CoinId, UserId};

pub mod event;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoinType {
    Paid,
    Free,
}

impl FromStr for CoinType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PAID" => Ok(CoinType::Paid),
            "FREE" => Ok(CoinType::Free),
            _ => Err(format!("Invalid coin type: {}", s)),
        }
    }
}

impl ToString for CoinType {
    fn to_string(&self) -> String {
        match self {
            CoinType::Paid => "PAID".to_string(),
            CoinType::Free => "FREE".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Coin {
    pub id: CoinId,
    pub user_id: UserId,
    pub session_id: Option<String>,
    pub purchased_amount: i32,
    pub used_amount: i32,
    pub coin_type: CoinType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Coin {
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();
        let expiration_date = self.created_at + Duration::days(180);
        now <= expiration_date
    }

    pub fn remaining_days(&self) -> i64 {
        let now = Utc::now();
        let expiration_date = self.created_at + Duration::days(180);
        let remaining = expiration_date - now;
        remaining.num_days()
    }

    pub fn remaining_amount(&self) -> i32 {
        self.purchased_amount - self.used_amount
    }
}

#[derive(Debug, Clone)]
pub struct CoinHistory {
    pub coin_id: CoinId,
    pub purchased_amount: i32,
    pub used_amount: i32,
    pub coin_type: CoinType,
    pub created_at: DateTime<Utc>,
    pub remaining_days: i64,
}
