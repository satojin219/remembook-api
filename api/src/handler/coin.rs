use std::str::FromStr;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use garde::Validate;
use kernel::model::coin::{
    event::{ConsumeCoin, CreateCoin},
    CoinType,
};
use registry::AppRegistry;
use shared::error::{AppError, AppResult};

use crate::{
    extractor::AuthorizedUser,
    model::coin::{CoinHistoryResponse, ConsumeCoinRequest, CreateCoinRequest, ValidCoinsResponse},
};

pub async fn create_coin(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<CreateCoinRequest>,
) -> Result<StatusCode, AppError> {
    req.validate(&())?;

    let coin = CreateCoin {
        user_id: user.id(),
        session_id: Some(req.session_id),
        amount: req.amount,
        coin_type: CoinType::from_str(&req.coin_type).unwrap(),
    };
    registry.coin_repository().create_coin(coin).await?;

    // 50、100枚の購入の時はボーナスで購入枚数の10%を無料で付与する
    if req.amount > 10 {
        let bonus_amount = req.amount / 10;
        let bonus_coin = CreateCoin {
            user_id: user.id(),
            session_id: None,
            amount: bonus_amount,
            coin_type: CoinType::Free,
        };
        registry.coin_repository().create_coin(bonus_coin).await?;
    }

    Ok(StatusCode::CREATED)
}

pub async fn consume_coin(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
    Json(req): Json<ConsumeCoinRequest>,
) -> Result<StatusCode, AppError> {
    let coin = ConsumeCoin {
        user_id: user.id(),
        amount: req.amount,
    };
    registry.coin_repository().consume_coin(coin).await?;
    Ok(StatusCode::OK)
}

pub async fn get_coin_history(
    State(registry): State<AppRegistry>,
    Path(user_id): Path<String>,
) -> Result<Json<Vec<CoinHistoryResponse>>, AppError> {
    let history = registry
        .coin_repository()
        .get_purchase_history_by_user_id(user_id.parse()?)
        .await?;
    let response = history
        .into_iter()
        .map(|coin| CoinHistoryResponse {
            coin_id: coin.coin_id.to_string(),
            purchased_amount: coin.purchased_amount,
            used_amount: coin.used_amount,
            coin_type: coin.coin_type.to_string(),
            created_at: coin.created_at.to_rfc3339(),
            remaining_days: coin.remaining_days,
        })
        .collect();
    Ok(Json(response))
}

pub async fn get_valid_coins(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<ValidCoinsResponse>> {
    let total_amount = registry
        .coin_repository()
        .get_valid_coins_by_user_id(user.id())
        .await?;

    Ok(Json(ValidCoinsResponse { total_amount }))
}

pub async fn get_purchase_history(
    user: AuthorizedUser,
    State(registry): State<AppRegistry>,
) -> AppResult<Json<Vec<CoinHistoryResponse>>> {
    let histories = registry
        .coin_repository()
        .get_purchase_history_by_user_id(user.id())
        .await?;

    let histories = histories
        .into_iter()
        .map(|h| CoinHistoryResponse {
            coin_id: h.coin_id.to_string(),
            purchased_amount: h.purchased_amount,
            used_amount: h.used_amount,
            coin_type: match h.coin_type {
                kernel::model::coin::CoinType::Paid => "PAID".to_string(),
                kernel::model::coin::CoinType::Free => "FREE".to_string(),
            },
            created_at: h.created_at.to_rfc3339(),
            remaining_days: h.remaining_days,
        })
        .collect();

    Ok(Json(histories))
}
