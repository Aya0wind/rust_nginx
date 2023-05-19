use serde::{Deserialize, Serialize};

fn log_level_default()->String{
    "info".to_string()
}

fn log_path_default()->String{
    ".".to_string()
}

fn log_format_default()->LogFormat{
    LogFormat::Text
}

#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogLevel{
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

#[derive(Debug,Clone,Serialize,Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LogFormat{
    Text,
    ColoredText,
    Json,
    Yaml,
    Xml,
    Csv,
    Custom{
        format:String,
    },
}

#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct LogDetailedConfig{
    #[serde(default = "log_level_default" ) ]
    pub level:String,
    #[serde(default = "log_path_default" ) ]
    pub path:String,
    #[serde(default = "log_format_default" ) ]
    pub format:LogFormat,
}
impl Default for LogDetailedConfig{
    fn default()->Self{
        Self{
            level:log_level_default(),
            path:log_path_default(),
            format:log_format_default(),
        }
    }
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct LogConfig{
    #[serde(default="log_level_default")]
    pub level:String,
    #[serde(default)]
    pub access_log:LogDetailedConfig,
    #[serde(default)]
    pub error_log:LogDetailedConfig,
    #[serde(default)]
    pub debug_log:LogDetailedConfig,
}

impl Default for LogConfig{
    fn default()->Self{
        Self{
            level:log_level_default(),
            access_log:LogDetailedConfig::default(),
            error_log:LogDetailedConfig::default(),
            debug_log:LogDetailedConfig::default(),
        }
    }
}


#[derive(Debug,Clone,Serialize,Deserialize,Default)]
pub struct LuaHooks{
    #[serde(default)]
    pub init:String,
    #[serde(default)]
    pub pre_access:String,
}

fn lua_enabled_default()->bool{
    true
}


#[derive(Debug,Clone,Serialize,Deserialize)]
pub struct LuaConfig{
    #[serde(default="lua_enabled_default")]
    pub enabled:bool,
    #[serde(default)]
    #[serde(flatten)]
    pub hooks: LuaHooks,
}

impl Default for LuaConfig{
    fn default()->Self{
        Self{
            enabled:lua_enabled_default(),
            hooks:LuaHooks::default(),
        }
    }
}

#[derive(Debug,Clone,Serialize,Deserialize,Default)]
pub struct CommonConfig{
    #[serde(default)]
    pub max_connection:u32,
    #[serde(default)]
    pub log:LogConfig,
    #[serde(default)]
    pub lua:LuaConfig,
}


