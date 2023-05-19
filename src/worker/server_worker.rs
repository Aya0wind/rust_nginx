use crate::config::ServerConfig;
use crate::error::Result;
pub struct ServerWorker{
    server_config:ServerConfig,
}


impl ServerWorker{
    pub fn new(server_config:ServerConfig)->Self{
        Self{server_config}
    }
    pub async fn run(&mut self)->Result<()>{
        Ok(())
    }
}