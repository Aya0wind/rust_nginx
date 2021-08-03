#[macro_use]
extern crate log;

use hyper::{Body, Client, Request, Response, Server};
use hyper::client::HttpConnector;
use hyper::Result;
use hyper::service::{make_service_fn, service_fn};
use mimalloc::MiMalloc;
use tokio::time::timeout_at;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;


mod router;
mod server;
async fn main_handle(
    req: Request<Body>,
) -> Result<Response<Body>> {
    trace!("Get Request:{:?}", req);
    router::params(&req);
    Ok(Response::new(Body::from("Hello")))
}


#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();
    let address=([127,0,0,1],80).into();
    let server = server::make_server(&address);
    let interrupt = tokio::signal::ctrl_c();
    //let handle = tokio::spawn(server1);
    //let res = handle.await;
    tokio::spawn(interrupt).await??;
    eprintln!("Server interrupted by ctrl-c signal\nBye");
    // tokio::select! {
    //     v =  =>{
    //         v??;
    //     }
    //     _ = tokio::spawn(interrupt) => {
    //         eprintln!("Server interrupted by ctrl-c signal\nBye");
    //     }
    // };
    Ok(())
}
