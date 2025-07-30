use std::net::SocketAddr;

use axum::Router;
use tokio::net::TcpListener;
use tracing::info;

use crate::{app::AppState, config::server::ServerConfig};

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
        Ok(Router::new().merge(router).with_state(state))
    }
}
