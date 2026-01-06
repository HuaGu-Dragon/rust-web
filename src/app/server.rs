use std::{
    net::{Ipv4Addr, SocketAddr, SocketAddrV4},
    time::Duration,
};

use axum::{Router, extract::Request, http::StatusCode};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::{
    cors::CorsLayer, limit::RequestBodyLimitLayer, normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer, trace::TraceLayer,
};
use tracing::info;
use uuid::Uuid;

use crate::{
    app::{AppState, latency::LatencyLayer},
    config::{self, server::ServerConfig},
};

pub struct Server {
    config: &'static ServerConfig,
}

impl Server {
    pub fn new(config: &'static ServerConfig) -> Self {
        Server { config }
    }

    pub async fn start(&self, router: Router<AppState>, state: AppState) -> anyhow::Result<()> {
        let router = self.build_router(router, state);
        let port = self.config.port();

        let address = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port));
        info!("listening on http://{}", address);

        let ssl = config::get().ssl();

        if ssl.enable() {
            let config = RustlsConfig::from_pem_file(
                ssl.cert_path().expect("read cert file"),
                ssl.key_path().expect("read key file"),
            )
            .await?;

            axum_server::bind_rustls(address, config)
                .serve(router.into_make_service_with_connect_info::<SocketAddr>())
                .await?;
        } else {
            axum_server::bind(address)
                .serve(router.into_make_service_with_connect_info::<SocketAddr>())
                .await?;
        }

        Ok(())
    }

    fn build_router(&self, router: Router<AppState>, state: AppState) -> Router {
        Router::new()
            .merge(router)
            .layer(NormalizePathLayer::trim_trailing_slash())
            .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024))
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
            .layer(CorsLayer::permissive())
            .layer(TimeoutLayer::with_status_code(
                StatusCode::REQUEST_TIMEOUT,
                Duration::from_secs(30),
            ))
            .with_state(state)
    }
}
