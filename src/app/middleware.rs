use std::pin::Pin;

use anyhow::Context;
use axum::{
    RequestExt,
    body::Body,
    http::{Request, Response},
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use tower_http::auth::AsyncAuthorizeRequest;

use crate::app::{auth, error::ApiError};

#[derive(Clone, Copy)]
pub struct AuthLayer;

impl AsyncAuthorizeRequest<Body> for AuthLayer {
    type RequestBody = Body;

    type ResponseBody = Body;

    type Future = Pin<
        Box<dyn Future<Output = Result<Request<Self::RequestBody>, Response<Self::ResponseBody>>>>,
    >;

    fn authorize(&mut self, mut request: Request<Body>) -> Self::Future {
        Box::pin(async move {
            let TypedHeader(Authorization(bearer)) = request
                .extract_parts::<TypedHeader<Authorization<Bearer>>>()
                .await
                .map_err(|e| ApiError::TypedHeaderError(e))?;

            let principal = auth::jwt_service().decode(bearer.token())?;

            request.extensions_mut().insert(principal);

            Ok(request)
        })
    }
}
