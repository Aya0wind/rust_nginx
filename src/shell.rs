use log::info;
use tokio::io::AsyncBufReadExt;
use tokio::sync::mpsc;

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
    let command = command.trim();
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


pub async fn shell_handler(shell_sender: mpsc::Sender<ShellCommand>) -> Result<()> {
    let mut command_reader = tokio::io::BufReader::new(tokio::io::stdin());
    let mut buf = vec![0u8; 256];
    loop {
        command_reader.read_until(b'\n', &mut buf).await?;
        let command = parse_command(&buf);
        if command == ShellCommand::Stop {
            break;
        }
        shell_sender.send(command).await.map_err(anyhow::Error::from)?;
    }
    info!("successfully start shell");
    Ok(())
}
