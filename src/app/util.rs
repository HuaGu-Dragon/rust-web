use std::sync::LazyLock;

use argon2::{
    Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::app::error::ApiResult;

static FAST_ARGON2: LazyLock<Argon2> = LazyLock::new(|| {
    let params = Params::new(4096, 1, 1, Some(32)).expect("Valid Argon2 params");

    Argon2::new(argon2::Algorithm::Argon2id, argon2::Version::V0x13, params)
});

pub fn hash_password_fast(password: &str) -> ApiResult<String> {
    Ok(FAST_ARGON2
        .hash_password(password.as_bytes(), &SaltString::generate(&mut OsRng))
        .map(|hash| hash.to_string())?)
}

pub fn verify_password(password: &str, hash: &str) -> ApiResult<bool> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(FAST_ARGON2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}
