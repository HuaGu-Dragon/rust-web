use axum::{http::Method, response::IntoResponse};
use rust_embed::Embed;

use crate::app::{error::ApiError, extract::Path};

#[derive(Embed)]
#[folder = "web/dist"]
#[include = "index.html"]
struct IndexHtml;

#[derive(Embed)]
#[folder = "web/dist"]
#[exclude = "index.html"]
struct StaticAssets;

struct StaticFile<T>(T);

impl<T: AsRef<str>> IntoResponse for StaticFile<T> {
    fn into_response(self) -> axum::response::Response {
        let path = self.0.as_ref();
        match StaticAssets::get(path) {
            Some(file) => {
                let mime = file.metadata.mimetype();

                ([(axum::http::header::CONTENT_TYPE, mime)], file.data).into_response()
            }
            None => ApiError::NotFound.into_response(),
        }
    }
}

pub async fn static_assets_handler(Path(path): Path<String>) -> impl IntoResponse {
    StaticFile(path).into_response()
}

pub async fn index_handler(method: Method) -> impl IntoResponse {
    if method == Method::GET {
        let file = IndexHtml::get("index.html")
            .expect("Index file should always be present in the embedded assets");
        let mime = file.metadata.mimetype();

        ([(axum::http::header::CONTENT_TYPE, mime)], file.data).into_response()
    } else {
        ApiError::MethodNotAllowed.into_response()
    }
}
