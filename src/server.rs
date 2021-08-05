use std::future::Future;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Body, Response};
use hyper::Request;
use hyper::Server;
use hyper::Result;
use hyper::server::conn::{AddrStream, AddrIncoming, Http};
use std::net::SocketAddr;
use std::sync::Arc;
use std::borrow::BorrowMut;
use mlua::Lua;


async fn main_handle(
    req: Request<Body>,
) -> hyper::Result<Response<Body>> {
    let lua = Lua::new();
    trace!("Get Request:{:?}", req);
    Ok(Response::new(Body::from(r##""##)))
}

struct Config;


struct ServerContext{
    config:Arc<Config>
}
impl ServerContext{
    fn update(&mut self){
        let boxed_config = Box::new(Config{});
    }
}

pub async fn make_server(
    address: SocketAddr
) ->impl Future<Output=Result<()>> {
    let make_service = make_service_fn(|_conn|{

        async { Ok::<_, hyper::Error>(service_fn(move |req|{

            main_handle(req)
        })) }
    });
    // Then bind and serve...
    Server::bind(&address).serve(make_service)
}


