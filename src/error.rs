use log::error;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("yaml config format invalid, caused by {0}")]
    YamlConfigFormat(#[from] serde_yaml::Error),
    #[error("json config format invalid, caused by {0}")]
    JsonConfigFormat(#[from] serde_json::Error),
    #[error("io error, caused by {0}")]
    IO(#[from] std::io::Error),
    #[error("proxy error, caused by {0}")]
    Http(#[from] hyper::Error),
    #[error("lua interpreter error, caused by {0}")]
    Lua(#[from] mlua::Error),
    #[error("unknown error")]
    Unknown,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub mod msg{
    pub const NO_AVAILABLE_ROUTE_RULE:&str="No available route rule";
}