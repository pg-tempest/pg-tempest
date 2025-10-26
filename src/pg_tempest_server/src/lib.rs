use std::{net::SocketAddrV4, sync::Arc};

use axum::Router;
use pg_tempest_core::PgTempestCore;
use tokio::net::TcpListener;

use crate::{
    configs::ServerConfigs,
    custom_trace_layer::custom_trace_layer,
    routes::{templates::create_templates_router, test_dbs::create_test_dbs_router},
};

pub mod configs;
mod custom_trace_layer;
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
            .layer(axum::middleware::from_fn(custom_trace_layer));

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
