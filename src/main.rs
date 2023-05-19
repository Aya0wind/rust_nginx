use crate::config::{Config};
use clap::Parser;
use log::{error, info};

mod lua;
mod error;
use error::Result;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use crate::server::Server;
use crate::shell::ShellCommand;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod config;
mod server;
mod proxy_server;
mod shell;
mod worker;
mod args;
#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init();
    let command_args = args::Args::parse();
    let (
        shell_sender,
        shell_receiver
    ) = mpsc::channel::<ShellCommand>(5);
    let config = Config::load_yaml_config_from_path(&command_args.config)?;
    info!("successfully loaded config file");
    let server_shutdown_token = CancellationToken::new();
    let server = Server::new(config, shell_receiver)?;
    let server_future = server.start(server_shutdown_token.clone());
    let shell_future = shell::shell_handler(shell_sender);
    let interrupt_signal= tokio::signal::ctrl_c();
    tokio::select!(
        server_result = server_future=>{
            if let Err(e) = server_result{
                error!("server error:{}",e);
            }
        },
        shell_result = shell_future=>{
            if let Err(e) = shell_result{
                error!("shell error:{}",e);
            }
            server_shutdown_token.cancel();
        },
        _ = interrupt_signal=>{
            info!("receive interrupt signal");
            server_shutdown_token.cancel();
        }
    );
    info!("server shutdown, bye");
    Ok(())
}
