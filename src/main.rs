use crate::config::{Config};
use clap::Parser;
use log::{error, info};

mod lua;
mod error;
use error::Result;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use crate::error::Error;
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
    let shell_future = shell::shell_handler(shell_sender,server_shutdown_token.clone());
    let interrupt_signal= shell::wait_for_sigint(server_shutdown_token.clone());
    let server_future = tokio::spawn(async move{
        let server = Server::new(config, shell_receiver)?;
        server.start(server_shutdown_token.clone()).await?;
        info!("server task exit");
        Ok::<_,Error>(())
    });
    tokio::join!(
        shell_future,
        interrupt_signal,
        server_future
    ).0?;
    info!("server exit! bye bye!");
    // because tokio::ioA::stdin() is impossible to be canceled
    // so we have to exit the process to break the blocking read
    // when call this exit, all the tasks except shell read will be canceled
    // so this operation is safe
    std::process::exit(0)
}
