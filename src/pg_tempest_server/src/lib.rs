use std::{net::SocketAddrV4, sync::Arc};

use axum::Router;
use pg_tempest_core::PgTempestCore;
use tokio::net::TcpListener;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{
    configs::ServerConfigs,
    routes::{templates::create_templates_router, test_dbs::create_test_dbs_router},
};

pub mod configs;
mod dtos;
mod routes;

pub struct Server {
    router: Router,
    configs: Arc<ServerConfigs>,
}

impl Server {
    pub fn new(tempest_core: Arc<PgTempestCore>, configs: Arc<ServerConfigs>) -> Server {
        let router = Router::new()
            .merge(create_templates_router(tempest_core.clone()))
            .merge(create_test_dbs_router(tempest_core.clone()))
            .layer(
                TraceLayer::new_for_http().on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
            );

        Server { router, configs }
    }

    pub async fn start(self) {
        let socket_addr = SocketAddrV4::new(self.configs.ipv4, self.configs.port);

        tracing::info!("Starting server on {socket_addr:?}");

        let tcp_listener = TcpListener::bind(socket_addr).await.unwrap();

        axum::serve(tcp_listener, self.router.into_make_service())
            .await
            .unwrap();
    }
}
