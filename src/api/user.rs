use crate::entity::{prelude::*, sys_user};
use anyhow::Context;
use axum::{Router, extract::State, routing};
use sea_orm::prelude::*;

use crate::app::{AppState, error::ApiResult, response::ApiResponse};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", routing::get(get_users))
}

async fn get_users(
    State(AppState { db }): State<AppState>,
) -> ApiResult<ApiResponse<Vec<sys_user::Model>>> {
    let users = SysUser::find()
        .all(&db)
        .await
        .context("Get users from database")?;
    Ok(ApiResponse::success(users))
}
