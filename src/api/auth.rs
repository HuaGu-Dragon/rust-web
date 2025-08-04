use std::net::SocketAddr;

use crate::app::auth::{Principal, jwt_service};
use crate::app::error::ApiError;
use crate::app::util::verify_password;
use crate::app::{ApiReturn, AppState, extract::ValidJson, response::ApiResponse};
use crate::entity::prelude::*;
use crate::entity::sys_user::{self};
use axum::extract::ConnectInfo;
use axum::{Router, extract::State, routing};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::Deserialize;
use tracing::info;
use validator::Validate;

pub fn create_router() -> Router<AppState> {
    Router::new().route("/login", routing::post(login))
}

#[derive(Clone, Deserialize, Validate)]
struct LoginParams {
    #[validate(length(
        min = 1,
        max = 16,
        message = "Account must be between 1 and 16 characters long"
    ))]
    pub account: String,
    #[validate(length(
        min = 6,
        max = 16,
        message = "Password must be between 6 and 16 characters long"
    ))]
    pub password: String,
}

#[tracing::instrument(name = "user_login", skip_all, fields(account = %params.account, ip = %addr.ip()))]
async fn login(
    State(AppState { db }): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ValidJson(params): ValidJson<LoginParams>,
) -> ApiReturn<String> {
    let user = SysUser::find()
        .filter(sys_user::Column::Account.eq(params.account))
        .one(&db)
        .await
        .map_err(|_| ApiError::LoginError)?;

    if user.is_none() || !verify_password(&params.password, &user.as_ref().unwrap().password)? {
        return Err(ApiError::LoginError);
    }

    let user = user.unwrap();
    let principal = Principal {
        id: user.id,
        name: user.name,
    };

    Ok(ApiResponse::success(jwt_service().encode(principal)?))
}
