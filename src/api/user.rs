use crate::{
    app::{
        ApiReturn,
        extract::{Path, ValidJson, ValidQuery},
        params::{Page, QueryParams},
        util::hash_password_fast,
    },
    entity::{
        gender::Gender,
        prelude::*,
        sys_user::{self, ActiveModel},
    },
};
use anyhow::Context;
use axum::{Router, extract::State, routing};
use sea_orm::{ActiveValue, Condition, IntoActiveModel, QueryOrder, QueryTrait, prelude::*};
use serde::Deserialize;
use validator::Validate;

use crate::app::{AppState, response::ApiResponse};

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/", routing::get(get_users).post(create_user))
        .route("/{id}", routing::put(update_user).delete(delete_user))
}

#[derive(Debug, Deserialize, Validate)]
struct UserQueryParams {
    keyword: Option<String>,
    #[validate(nested)]
    #[serde(flatten)]
    pagination: QueryParams,
}

#[derive(Debug, Clone, Deserialize, Validate, DeriveIntoActiveModel)]
struct UserParams {
    #[validate(length(
        min = 1,
        max = 16,
        message = "Name must be between 1 and 16 characters long"
    ))]
    pub name: String,
    pub gender: Gender,
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
    #[validate(custom(function = "crate::app::validation::validate_mobile_phone"))]
    pub mobile_phone: String,
    pub birthday: Date,
    #[serde(default)]
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize, Validate)]
struct UpdateUserParams {
    #[validate(length(
        min = 1,
        max = 16,
        message = "Name must be between 1 and 16 characters long"
    ))]
    pub name: Option<String>,
    pub gender: Option<Gender>,
    #[validate(length(
        min = 1,
        max = 16,
        message = "Account must be between 1 and 16 characters long"
    ))]
    pub account: Option<String>,
    #[validate(length(
        min = 6,
        max = 16,
        message = "Password must be between 6 and 16 characters long"
    ))]
    pub password: Option<String>,
    #[validate(custom(function = "crate::app::validation::validate_mobile_phone"))]
    pub mobile_phone: Option<String>,
    pub birthday: Option<Date>,
    pub enabled: Option<bool>,
}

async fn create_user(
    State(AppState { db }): State<AppState>,
    ValidJson(user_params): ValidJson<UserParams>,
) -> ApiReturn<sys_user::Model> {
    let mut active_model = user_params.into_active_model();

    active_model.password = ActiveValue::set(hash_password_fast(active_model.password.as_ref())?);

    Ok(ApiResponse::success(
        active_model.insert(&db).await.context("Create user")?,
    ))
}

macro_rules! update_params {
    ($active_model:expr, $field:ident, $value:expr) => {
        if let Some(value) = $value {
            $active_model.$field = ActiveValue::Set(value);
        }
    };
}

async fn update_user(
    State(AppState { db }): State<AppState>,
    Path(user_id): Path<String>,
    ValidJson(user_params): ValidJson<UpdateUserParams>,
) -> ApiReturn<sys_user::Model> {
    let user = SysUser::find_by_id(user_id)
        .one(&db)
        .await
        .context("Find User")?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    let mut active_model = user.into_active_model();

    update_params!(active_model, name, user_params.name);
    update_params!(active_model, gender, user_params.gender);
    update_params!(active_model, account, user_params.account);
    update_params!(active_model, mobile_phone, user_params.mobile_phone);
    update_params!(active_model, birthday, user_params.birthday);
    update_params!(active_model, enabled, user_params.enabled);

    if let Some(password) = user_params.password {
        active_model.password = ActiveValue::Set(hash_password_fast(&password)?);
    }

    Ok(ApiResponse::success(
        active_model.update(&db).await.context("Update user")?,
    ))
}

async fn delete_user(
    State(AppState { db }): State<AppState>,
    Path(user_id): Path<String>,
) -> ApiReturn<()> {
    let user = SysUser::find_by_id(&user_id)
        .one(&db)
        .await
        .context("Find user by user_id")?
        .ok_or(anyhow::anyhow!("User not found"))?;

    let result = user.delete(&db).await.context("Delete the user")?;
    tracing::info!(
        "Delete User: {user_id}, rows_affected: {}",
        result.rows_affected
    );

    Ok(ApiResponse::success(()))
}

async fn get_users(
    State(AppState { db }): State<AppState>,
    ValidQuery(UserQueryParams {
        keyword,
        pagination,
    }): ValidQuery<UserQueryParams>,
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
