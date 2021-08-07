use hyper::http::uri::Uri;
use hyper::{Request, Body};
use std::panic::panic_any;


pub fn params(req:&Request<Body>){
   let path =std::path::Path::new(req.uri().path());
    if(path.strip_prefix())

    eprintln!("{:?}",req.uri().path());
}