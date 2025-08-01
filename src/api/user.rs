use crate::{
    app::{
        ApiReturn,
        params::{Page, QueryParams},
    },
    entity::{prelude::*, sys_user},
};
use anyhow::Context;
use axum::{
    Router,
    extract::{Query, State},
    routing,
};
use sea_orm::{Condition, QueryOrder, QueryTrait, prelude::*};
use serde::Deserialize;

use crate::app::{AppState, response::ApiResponse};

pub fn create_router() -> Router<AppState> {
    Router::new().route("/", routing::get(get_users))
}

#[derive(Debug, Deserialize)]
struct UserQueryParams {
    keyword: Option<String>,
    #[serde(flatten)]
    pagination: QueryParams,
}

async fn get_users(
    State(AppState { db }): State<AppState>,
    Query(UserQueryParams {
        keyword,
        pagination,
    }): Query<UserQueryParams>,
) -> ApiReturn<Page<sys_user::Model>> {
    let paginator = SysUser::find()
        .apply_if(keyword.as_ref(), |query, keyword| {
            query.filter(
                Condition::any()
                    .add(sys_user::Column::Name.contains(keyword))
                    .add(sys_user::Column::Account.contains(keyword))
                    .add(sys_user::Column::MobilePhone.contains(keyword))
                    .add(sys_user::Column::Gender.contains(keyword)),
            )
        })
        .order_by_desc(sys_user::Column::CreatedAt)
        .paginate(&db, pagination.page_size);

    let size = paginator
        .num_items()
        .await
        .context("Failed to get number of items")?;

    let users = paginator
        .fetch_page(pagination.page - 1)
        .await
        .context("Failed to fetch users")?;

    Ok(ApiResponse::success(Page::from_pagination(
        pagination, size, users,
    )))
}
