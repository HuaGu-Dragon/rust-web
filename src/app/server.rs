use std::net::SocketAddr;

use axum::{Router, extract::Request};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use uuid::Uuid;

use crate::{
    app::{AppState, latency::LatencyLayer},
    config::server::ServerConfig,
};

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Server { config }
    }

    pub async fn start(&self, router: Router<AppState>, state: AppState) -> anyhow::Result<()> {
        let router = self.build_router(router, state)?;
        let port = self.config.port();

        let listener = TcpListener::bind(format!("0.0.0.0:{port}")).await?;
        info!("listening on {}", listener.local_addr()?);

        axum::serve(
            listener,
            router.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        Ok(())
    }

    fn build_router(&self, router: Router<AppState>, state: AppState) -> anyhow::Result<Router> {
        Ok(Router::new()
            .merge(router)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request| {
                        let id = Uuid::new_v4();
                        tracing::info_span!(
                            "http_request",
                            id = %id,
                            method = %request.method(),
                            uri = %request.uri(),
                            version = ?request.version(),
                        )
                    })
                    .on_request(())
                    .on_response(LatencyLayer)
                    .on_failure(()),
            )
            .with_state(state))
    }
}
