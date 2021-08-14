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
use futures::pin_mut;
use crate::config::{Config, GlobalConfig, ServerConfig};


async fn main_handle(
    req: Request<Body>,
    client: Client<HttpConnector>,
    config:ServerInstance
) -> hyper::Result<Response<Body>> {
    trace!("Get Request:{:?}", req);

    Ok(Response::new(Body::from(r##"Hello world"##)))
}

#[derive(Debug,Clone)]
struct ServerInstance {
    global_config: GlobalConfig,
    config: ServerConfig,
}

#[derive(Debug,Clone)]
pub struct Server{
    global:GlobalConfig,
    servers:Vec<ServerConfig>,
}

impl Server{

    fn build_runner(address: SocketAddr,config:ServerInstance, mut control_channel: tokio::sync::broadcast::Receiver<()>) ->impl Future<Output = Result<()>>{
        let make_service = make_service_fn(move |_conn| {
            let client=Client::new();
            let config = config.clone();
            async {
                Ok::<_, hyper::Error>(service_fn(move |req| main_handle(req, client.clone(),config.clone())))
            }
        });
        // Then bind and serve...
        hyper::Server::bind(&address).serve(make_service).with_graceful_shutdown(
            async move{
                control_channel.recv().await;
            }
        )
    }

    pub fn new(config: Config) -> Self{
        let servers = config.servers;
        let global = config.global;
        Self{servers,global}
    }

   pub async fn run(&mut self) ->Result<()>{
       let (sx,rx) = tokio::sync::broadcast::channel(8);
        let instances = self.servers
            .iter()
            .map(|c| {
                let instance = ServerInstance{global_config:self.global.clone(),config:c.clone()};
                let runner =Self::build_runner(c.bind_address,instance,sx.subscribe());
                Box::pin(runner)
            });
       futures::future::select_all(instances).await.0
    }
}

