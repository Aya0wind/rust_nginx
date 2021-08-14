use std::net::SocketAddr;
#[derive(Debug,Clone)]
pub struct GlobalConfig {

}

#[derive(Debug,Clone)]
pub enum Scheme{
    HTTP,
    HTTPS
}
#[derive(Debug,Clone)]
pub struct ServerConfig{
    pub bind_address:SocketAddr,
    pub scheme:Scheme
}

#[derive(Debug,Clone)]
pub struct Config{
    pub global:GlobalConfig,
    pub servers:Vec<ServerConfig>,
}