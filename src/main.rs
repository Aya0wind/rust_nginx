#[macro_use]
extern crate log;

use hyper::{Body, Client, Request, Response, Server};
use hyper::client::HttpConnector;
use hyper::Result;
use hyper::service::{make_service_fn, service_fn};
use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

mod config_parser;
mod router;

async fn main_handle(
    mut req: Request<Body>,
    target_server: Client<HttpConnector>,
) -> Result<Response<Body>> {
    trace!("Get Request:{:?}", req);
    let resq =
    Ok(resp)
}


#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    pretty_env_logger::init();

    let client = Client::new();
    // For every connection, we must make a `Service` to handle all
    // incoming HTTP requests on said connection.
    let make_svc = make_service_fn(|_conn| {
        // This is the `Service` that will handle the connection.
        // `service_fn` is a helper to convert a function that
        // returns a Response into a `Service`.
        let service = service_fn(move |req| main_handle(req, client));

        // Return the service to hyper.
        async move { Ok::<_,hyper::Error>(service) }
    });

    let address = ([127, 0, 0, 1], 80).into();

    let server1 = Server::bind(&address).serve(make_svc);

    let interrupt = tokio::signal::ctrl_c();


    tokio::select! {
        v = server1 =>{
            v?;
        }
        _ = interrupt => {
            eprintln!("Server interrupted by ctrl-c signal\nBye");
        }
    };

    Ok(())
}
