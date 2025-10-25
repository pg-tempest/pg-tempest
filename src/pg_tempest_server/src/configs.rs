use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Deserialize)]
pub struct ServerConfigs {
    pub ipv4: Ipv4Addr,
    pub port: u16,
}
