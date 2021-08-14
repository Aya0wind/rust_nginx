use hyper::{Body, Client, Response, Uri};
use hyper::client::HttpConnector;

use crate::error::Error::ProxyError;
use crate::error::Result;

pub enum Resolver {
    StaticResource(String),
    RemoteResource(Uri),
}


impl Resolver {
    pub async fn request(&self, client: Client<HttpConnector>) -> Result<Response<Body>> {
        match self {
            Resolver::StaticResource(path) => {}
            Resolver::RemoteResource(uri) => {}
        }
        hyper::Error
    }

    async fn get_response_from_uri(uri: Uri, client: hyper::Client<HttpConnector>) -> Result<Response<Body>> {
        client.request(&uri).await.map_err(ProxyError)
    }

    async fn get_response_from_static_file(uri: Uri, client: hyper::Client<HttpConnector>) -> {
        let result = client.request(uri).await?;
    }
}