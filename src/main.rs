#[macro_use]
extern crate log;

use mimalloc::MiMalloc;

use tokio::task::LocalSet;
use crate::config::{Config, GlobalConfig, ServerConfig, Scheme};
use crate::server::Server;
use futures::future::{Either};
use mlua::Lua;
use std::sync::{Mutex,Arc};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

// mod router;
// mod error;
mod server;
mod config;
// mod resource_resolver;
mod shell;

std::thread_local! {

}

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    let lua:Arc<Mutex<Lua>> =Arc::new(Mutex::new(Lua::new()));
    let config = Config{ global: GlobalConfig {lua}, servers: vec![
        ServerConfig{ bind_address: ([127,0,0,1],80).into(), scheme: Scheme::HTTP },
        ServerConfig{ bind_address: ([127,0,0,1],81).into(), scheme: Scheme::HTTP },
        ServerConfig{ bind_address: ([127,0,0,1],82).into(), scheme: Scheme::HTTP },
        ServerConfig{ bind_address: ([127,0,0,1],83).into(), scheme: Scheme::HTTP },
    ] };
    let mut server = Server::new(config);
    let local_tasks = LocalSet::new();
    tokio::select! {
        server_result=server.run()=>{
            eprintln!("error exit!:{:?}",server_result);
        }
        shell_result=local_tasks.run_until(shell::shell_handler())=>{
            eprintln!("shell exit!:{:?}",shell_result);
        }
    }
    Ok(())
}
