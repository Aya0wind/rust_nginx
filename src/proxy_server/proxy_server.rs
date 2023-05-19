use std::future::Future;
use std::net::SocketAddr;
use std::sync::Arc;
use std::task::{Context, Poll};

use hyper::client::HttpConnector;
use hyper::server::conn::{AddrIncoming, AddrStream, Http};
use hyper::service::{make_service_fn, service_fn};
use hyper::Request;
use hyper::Result;
use hyper::{Body, Client, Response};
use crate::config::{Config, ServerConfig};
use crate::worker::ServerWorker;

#[derive(Debug,Clone)]
struct ProxyServer {
    config: ServerConfig,
}

#[derive(Debug,Clone)]
pub struct Server{
    //servers:Vec<ServerWorker>,
}

impl Server{


    pub fn new(config:Config){

    }


   pub async fn run(&mut self) ->Result<()>{
       // let (sx,rx) = tokio::sync::broadcast::channel(8);
       //  let instances = self.servers
       //      .iter()
       //      .map(|c| {
       //          let instance = ServerInstance{global_config:self.global.clone(),config:c.clone()};
       //          let runner =Self::build_runner(c.bind_address,instance,sx.subscribe());
       //          Box::pin(runner)
       //      });
       // futures::future::select_all(instances).await.0
       Ok(())
    }

    pub async fn reload_config(&mut self)->Result<()>{
        Ok(())
    }
}

