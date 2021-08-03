use std::future::Future;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Body, Response};
use hyper::Request;
use hyper::Server;
use hyper::Result;
use hyper::server::conn::{AddrStream, AddrIncoming, Http};
use std::net::SocketAddr;


async fn main_handle(
    req: Request<Body>,
) -> hyper::Result<Response<Body>> {
    trace!("Get Request:{:?}", req);
    Ok(Response::new(Body::from("Hello")))
}

pub async fn make_server(
    address: &SocketAddr,
) -> Box<dyn Future<Output=Result<()>>> {
    let make_service = make_service_fn(|_conn| async {
        Ok::<_, hyper::Error>(service_fn(main_handle))
    });
    // Then bind and serve...
    Box::new(Server::bind(&address).serve(make_service))
}
