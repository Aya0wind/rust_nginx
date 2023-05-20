use log::info;
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::error::Result;

#[derive(Debug, PartialEq)]
pub enum ShellCommand {
    Stop,
    Reload(String),
    Status,
    Unknown,
}


fn parse_command(bytes: &[u8]) -> ShellCommand {
    let command = String::from_utf8_lossy(bytes);
    let command = command.trim().trim_matches('\n').trim_matches('\r');
    match command {
        "stop" => ShellCommand::Stop,
        "status" => ShellCommand::Status,
        _ => {
            if command.starts_with("reload") {
                let mut command = command.split_whitespace();
                command.next();
                let config_path = command.next().unwrap_or("");
                ShellCommand::Reload(config_path.to_string())
            } else {
                ShellCommand::Unknown
            }
        }
    }
}


pub async fn shell_handler(
    shell_sender: mpsc::Sender<ShellCommand>,
    server_shutdown_token: CancellationToken,
) -> Result<()> {
    let mut command_reader = tokio::io::BufReader::new(tokio::io::stdin());
    let mut buf = vec![];
    info!("successfully start shell");
    loop {
        tokio::select! {
            _ = server_shutdown_token.cancelled() => {
                info!("server shutdown");
                break;
            }
             _ = command_reader.read_until(b'\n', &mut buf)=>{
                 let command = parse_command(&buf);
                if command == ShellCommand::Stop {
                    info!("received stop command");
                    server_shutdown_token.cancel();
                }
                shell_sender.send(command).await.map_err(anyhow::Error::from)?;
            }
        }
    }
    Ok(())
}

pub async fn wait_for_sigint(
    server_shutdown_token: CancellationToken,
)-> Result<()> {
    let sigint_stream = tokio::signal::ctrl_c();
    tokio::select! {
        _ = server_shutdown_token.cancelled() => {
            info!("server shutdown, stop receive SIGINT");
        }
        err = sigint_stream => {
            err?;
            info!("received SIGINT");
            server_shutdown_token.cancel();
        }
    }
    Ok(())
}