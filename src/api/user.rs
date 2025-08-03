use std::sync::LazyLock;

use crate::{
    app::{
        ApiReturn,
        extract::{Path, ValidJson, ValidQuery},
        params::{Page, QueryParams},
    },
    entity::{
        gender::Gender,
        prelude::*,
        sys_user::{self, ActiveModel},
    },
};
use anyhow::Context;
use argon2::{
    Argon2, Params, PasswordHasher,
    password_hash::{SaltString, rand_core::OsRng},
};
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

static FAST_ARGON2: LazyLock<Argon2> = LazyLock::new(|| {
    let params = Params::new(4096, 1, 1, Some(32)).expect("Valid Argon2 params");

    Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params)
});

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

pub fn hash_password_fast(password: &str) -> Result<String, argon2::password_hash::Error> {
    FAST_ARGON2
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map(|hash| hash.to_string())
}

// pub fn verify_password(password: &str, hash: &str) -> Result<bool, argon2::password_hash::Error> {
//     use argon2::PasswordVerifier;
//     use argon2::password_hash::PasswordHash;

//     let parsed_hash = PasswordHash::new(hash)?;
//     Ok(FAST_ARGON2
//         .verify_password(password.as_bytes(), &parsed_hash)
//         .is_ok())
// }

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

async fn update_user(
    State(AppState { db }): State<AppState>,
    Path(user_id): Path<String>,
    ValidJson(user_params): ValidJson<UserParams>,
) -> ApiReturn<sys_user::Model> {
    let user = SysUser::find_by_id(user_id)
        .one(&db)
        .await
        .context("Find User")?
        .ok_or_else(|| anyhow::anyhow!("User not found"))?;

    let mut active_model = user.into_active_model();

    active_model.name = ActiveValue::Set(user_params.name);
    active_model.gender = ActiveValue::Set(user_params.gender);
    active_model.account = ActiveValue::Set(user_params.account);
    active_model.mobile_phone = ActiveValue::Set(user_params.mobile_phone);
    active_model.birthday = ActiveValue::Set(user_params.birthday);
    active_model.enabled = ActiveValue::Set(user_params.enabled);

    if !user_params.password.is_empty() {
        active_model.password = ActiveValue::Set(hash_password_fast(&user_params.password)?);
    } else {
        active_model.not_set(sys_user::Column::Password);
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
