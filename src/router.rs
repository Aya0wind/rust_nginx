use hyper::http::uri::Uri;
use hyper::{Request, Body};


pub fn params(req:&Request<Body>){
    eprintln!("{:?}",req.uri().path());
}