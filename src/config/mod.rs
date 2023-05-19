use std::io::BufReader;
use std::path::Path;
use serde::{Deserialize, Serialize};
mod server_config;
mod common_config;
pub use server_config::*;
pub use common_config::*;
use crate::error::Result;
#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct Config{
    #[serde(default)]
    pub servers:ServerConfig,
    #[serde(default)]
    #[serde(flatten)]
    pub common:CommonConfig,
}

impl Config{
    pub fn load_yaml_config_from_path<P:AsRef<Path>>(path:P)->Result<Self>{
        let reader = BufReader::new(std::fs::File::open(path)?);
        serde_yaml::from_reader::<_,Self>(reader).map_err(Into::into)
    }
    pub fn load_yaml_config_from_str(content:&str)->Result<Self>{
        serde_yaml::from_str::<Self>(content).map_err(Into::into)
    }

    pub fn load_json_config_from_str(content:&str)->Result<Self>{
        serde_yaml::from_str::<Self>(content).map_err(Into::into)
    }

    pub fn load_json_config_from_path<P:AsRef<Path>>(path:P)->Result<Self>{
        let reader = BufReader::new(std::fs::File::open(path)?);
        serde_json::from_reader::<_,Self>(reader).map_err(Into::into)
    }
}


mod test{
    use crate::config::Config;
    #[test]
    fn test_load_yaml_config() {
        let config_content = include_str!("../../rnginx_conf.yaml");
        let config = Config::load_yaml_config_from_str(config_content).unwrap();
        println!("{:#?}",config);
    }
    #[test]
    fn test_load_json_config() {
        let config_content = include_str!("../../rnginx_conf.json");
        let config = Config::load_json_config_from_str(config_content).unwrap();
        println!("{:#?}",config);
    }
}