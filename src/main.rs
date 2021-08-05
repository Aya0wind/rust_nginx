#[macro_use]
extern crate log;

use mimalloc::MiMalloc;
use std::collections::HashMap;
use mlua::Lua;
use std::sync::{Arc, Mutex};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;


mod router;
mod server;


fn main() -> std::result::Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rt  = tokio::runtime::Builder::new_multi_thread().worker_threads(16).enable_all().build()?;
    rt.block_on(async {
        let mut context=HashMap::<i32,i32>::new();


        pretty_env_logger::init();
        let address0=([127,0,0,1],80).into();
        let address1=([127,0,0,1],81).into();
        let server0 = server::make_server(address0);
        let handle0 = tokio::spawn(server0.await);
        let server1 = server::make_server(address1);
        let handle1 = tokio::spawn(server1.await);
        let mut vec = vec![handle0];
        vec.push(handle1);
        let interrupt = tokio::signal::ctrl_c();

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
    })
}
