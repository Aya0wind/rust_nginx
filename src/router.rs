use hyper::{Body, Client, Request, Response};
use hyper::client::HttpConnector;
use hyper::http::uri::Uri;

use crate::error::{Error, Result};
use crate::resource_resolver::Resolver;

pub struct ProxyRules {
    location: String,
    root_catalog: Option<String>,
    proxy: Option<String>,
    index: Option<String>,
    target: Option<String>,
}

pub struct Router<'a> {
    rules: &'a Vec<ProxyRules>,
}

enum Destination {
    Remote(Uri),
    Local(String),
}

impl<'a> Router<'a> {
    fn filter(req_uri: &Uri, rules: &ProxyRules) -> bool {
        let location = &rules.location;
        if req_uri.path().starts_with(location) {
            true
        } else {
            false
        }
    }

    pub async fn route(self, req_uri: &Uri) -> Result<Destination> {
        let rule = self.rules.iter().find(|rule| Self::filter(req_uri, rule));
        if let Some(rule) = rule {
            if let Some(path) = &rule.root_catalog {
                Ok(Destination::Local(Self::route_to_static_resources(req_uri, rule)?))
            } else {
                Ok(Destination::Remote(Self::route_to(req_uri, rule)?))
            }
        } else {
            Err(Error::NoAvailableRouteRule(req_uri.clone()))
        }
    }

    fn route_to(req_uri: &Uri, rules: &ProxyRules) -> Result<Uri> {
        let mut uri_builder = Uri::builder().scheme("https");
        if let Some(proxy) = &rules.proxy {
            uri_builder.authority(proxy.as_str());
        }
        uri_builder.path_and_query(req_uri.path_and_query().unwrap().as_str());
        uri_builder.build().map_err(|e| Error::NoAvailableRouteRule(req_uri.clone()))
    }

    fn route_to_static_resources(req_uri: &Uri, rules: &ProxyRules) -> Result<String> {
        if let Some(root) = &rules.root_catalog {}
        if let Some(proxy) = &rules.proxy {
            uri_builder.authority(proxy.clone());
        }
    }
}

