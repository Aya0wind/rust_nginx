use hyper::Uri;

pub enum Error {
    NoAvailableRouteRule(Uri),
    InvalidUri(hyper::http::Error),
    ProxyError(hyper::Error),
}


impl From<hyper::http::Error> for Error {
    fn from(err: hyper::http::Error) -> Self {
        Error::InvalidUri(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::ProxyError(err)
    }
}

pub type Result<T> = std::result::Result<T, Error>;