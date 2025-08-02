use axum::{
    extract::{FromRequest, FromRequestParts, Request},
    http::request::Parts,
};
use axum_valid::HasValidate;

use crate::app::error::ApiError;

macro_rules! impl_validate {
    ($name:ident) => {
        impl<T> HasValidate for $name<T> {
            type Validate = T;

            fn get_validate(&self) -> &Self::Validate {
                &self.0
            }
        }
    };
}

macro_rules! impl_validate_request {
    ($name:ident, $type:ident, FromRequestParts) => {
        impl<S, T> FromRequestParts<S> for $name<T>
        where
            S: Send + Sync,
            Valid<$type<T>>: FromRequestParts<S, Rejection = ApiError>,
        {
            #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
            #[doc = " a kind of error that can be converted into a response."]
            type Rejection = ApiError;

            #[doc = " Perform the extraction."]
            async fn from_request_parts(
                parts: &mut Parts,
                state: &S,
            ) -> Result<Self, Self::Rejection> {
                let inner = Valid::from_request_parts(parts, state).await?;
                Ok(Self(inner.0.0))
            }
        }
    };
    ($name:ident, $type:ident, FromRequest) => {
        impl<T, S> FromRequest<S> for $name<T>
        where
            S: Send + Sync,
            Valid<$type<T>>: FromRequest<S, Rejection = ApiError>,
        {
            #[doc = " If the extractor fails it\'ll use this \"rejection\" type. A rejection is"]
            #[doc = " a kind of error that can be converted into a response."]
            type Rejection = ApiError;

            #[doc = " Perform the extraction."]
            async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
                let inner = Valid::from_request(req, state).await?;
                Ok(Self(inner.0.0))
            }
        }
    };
}

#[derive(Debug, FromRequestParts)]
#[from_request(via(axum::extract::Path), rejection(ApiError))]
pub struct Path<T>(pub T);

impl_validate!(Path);

#[derive(Debug, Clone, Copy, Default, FromRequestParts)]
#[from_request(via(axum::extract::Query), rejection(ApiError))]
pub struct Query<T>(pub T);

impl_validate!(Query);

#[derive(Debug, Clone, Copy, Default, FromRequest)]
#[from_request(via(axum::extract::Json), rejection(ApiError))]
pub struct Json<T>(pub T);

impl_validate!(Json);

#[derive(Debug, Clone, Copy, Default, FromRequest, FromRequestParts)]
#[from_request(via(axum_valid::Valid), rejection(ApiError))]
pub struct Valid<T>(pub T);

#[derive(Debug)]
pub struct ValidPath<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidQuery<T>(pub T);

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidJson<T>(pub T);

impl_validate_request!(ValidQuery, Query, FromRequestParts);
impl_validate_request!(ValidPath, Path, FromRequestParts);
impl_validate_request!(ValidJson, Json, FromRequest);
