use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use crate::config::Config;
use crate::error::Result;
use crate::shell::ShellCommand;

pub struct Server{
    config:Config,
    command_channel:mpsc::Receiver<ShellCommand>,
}

impl Server{
    pub fn new(
        config:Config,
        command_channel:mpsc::Receiver<ShellCommand>
    )->Result<Self>{
        Ok(Self{
            config,
            command_channel
        })
    }

    pub async fn start(&self,server_shutdown_token:CancellationToken,)->Result<()>{
        tokio::select!(
            canceled = server_shutdown_token.cancelled()=>{

            },
            _ =tokio::time::sleep(tokio::time::Duration::from_secs(3))=>{

            }
        );
        Ok(())
    }
}