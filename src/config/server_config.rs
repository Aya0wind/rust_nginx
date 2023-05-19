use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::config::common_config::LuaConfig;

// #[derive(Debug,Clone,Serialize,Deserialize)]
// pub struct UdpServerConfig{
//     server_config:ServerConfig,
// }
//
//
// #[derive(Debug,Clone,Serialize,Deserialize)]
// pub struct QuicServerConfig{
//     server_config:ServerConfig,
// }


fn listen_ip_default()->String {
    "0.0.0.0".to_string()
}

fn listen_port_default()->u16{
    80
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct TcpServerConfig{
    #[serde(default = "listen_ip_default")]
    pub ip:String,
    #[serde(default = "listen_port_default")]
    pub port:u16,
    pub tcp_proxy:Option<String>,
}


impl Default for TcpServerConfig{
    fn default()->Self{
        Self{
            ip:listen_ip_default(),
            port:listen_port_default(),
            tcp_proxy:None,
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(untagged)]
pub enum HttpProxy{
    List(Vec<String>),
    Single(String),
    SubRule(HashMap<String,String>)
}

#[derive(Debug,Clone,Serialize,Deserialize,Default)]
pub struct HttpServerConfig{
    #[serde(default)]
    pub tcp:TcpServerConfig,
    #[serde(default)]
    pub error_page:HashMap<String,String>,
    #[serde(default)]
    pub lua:LuaConfig,
    #[serde(default)]
    //TODO: use prefix index to store the proxy rules
    pub http_proxy:HashMap<String,HttpProxy>,
}

#[derive(Debug,Clone,Serialize,Deserialize,Default)]
pub struct ServerConfig{
    #[serde(default)]
    pub http:HashMap<String,HttpServerConfig>,
    #[serde(default)]
    pub tcp:HashMap<String,TcpServerConfig>,
    // #[serde(default)]
    // pub udp:HashMap<String,UdpServerConfig>,
    // #[serde(default)]
    // pub quic:HashMap<String,QuicServerConfig>,
}
