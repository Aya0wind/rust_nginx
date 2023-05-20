use std::collections::HashMap;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;
use crate::config::Config;
use crate::error::{Error, Result};
use crate::shell::ShellCommand;
use crate::worker;

pub struct Server{
    config:Config,
    command_channel:mpsc::Receiver<ShellCommand>,
    worker_tasks:HashMap<String,tokio::task::JoinHandle<Result<()>>>,
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

    fn create_worker_task(&mut self,worker_name:String,shutdown_token:CancellationToken)->Result<()>{
        let worker_config = self.config.get_worker_config(&worker_name)?;
        let worker = worker::ServerWorker::new(worker_config)?;
        let worker_task = tokio::spawn(async move{
            worker.run().await?;
            Ok::<_,Error>(())
        });
        self.worker_tasks.insert(worker_name,worker_task);
        Ok(())
    }

    fn create_all_worker_tasks(&mut self,shutdown_token:CancellationToken)->Result<()>{
        for worker_name in self.config.get_all_worker_names(){
            self.create_worker_task(worker_name.clone(),shutdown_token.clone())?;
        }
        Ok(())
    }


    pub async fn start(&self,server_shutdown_token:CancellationToken,)->Result<()>{

        futures::future::join_all(self.worker_tasks.values()).await;
        Ok(())
    }
}