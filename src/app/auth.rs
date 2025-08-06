use std::sync::LazyLock;

use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, get_current_timestamp};
use serde::{Deserialize, Serialize};

use crate::{
    app::error::{ApiError, ApiResult},
    config,
};

#[derive(Debug, Clone, Serialize)]
pub struct Principal {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: u64,
    iat: u64,
}

static JWT_SERVICE: LazyLock<JwtService> = LazyLock::new(JwtService::new);

pub struct JwtService {
    encode_key: EncodingKey,
    decode_key: DecodingKey,
    header: Header,
    validation: Validation,
    expiration: u64,
}

impl JwtService {
    pub fn new() -> Self {
        let config = config::get().auth();
        let mut validation = Validation::new(config.algorithm());
        validation.set_required_spec_claims(&["sub", "exp", "iat"]);

        Self {
            encode_key: EncodingKey::from_secret(config.secret().as_bytes()),
            decode_key: DecodingKey::from_secret(config.secret().as_bytes()),
            header: Header::new(config.algorithm()),
            validation,
            expiration: config.expiration(),
        }
    }

    pub fn encode(&self, principal: Principal) -> ApiResult<String> {
        let now = get_current_timestamp();

        let claims = Claims {
            sub: format!("{}:{}", principal.id, principal.name),
            exp: now.saturating_add(self.expiration),
            iat: now,
        };

        Ok(jsonwebtoken::encode(
            &self.header,
            &claims,
            &self.encode_key,
        )?)
    }

    pub fn decode(&self, token: &str) -> ApiResult<Principal> {
        let token_data =
            jsonwebtoken::decode::<Claims>(token, &self.decode_key, &self.validation)?.claims;

        let (id, name) = token_data
            .sub
            .split_once(':')
            .ok_or_else(|| ApiError::ValidationError("Invalid token subject format".to_string()))?;

        Ok(Principal {
            id: id.to_string(),
            name: name.to_string(),
        })
    }
}

pub fn jwt_service() -> &'static JwtService {
    &JWT_SERVICE
}
