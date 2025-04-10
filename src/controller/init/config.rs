use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    pub(crate) log: Log,
    pub web: Web,
    pub clickhouse: Clickhouse,
    pub mysql: Mysql,
    pub url: Url,
}

///url
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Url {
    pub cabinet: String,
}

/// clickhouse
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Clickhouse {
    pub host: String,
}

///mysql
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Mysql {
    pub host: String,
    pub max_open: u64,
    pub idle_open: u64,
    pub max_life_time: u64,
    pub timeout: u64,
}

/// web config
#[derive(Debug, Serialize, Deserialize)]
pub struct Web {
    pub host: String,
    pub max_age: usize,
    pub worker_num: usize,
    pub shutdown_timeout: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub debug: bool,
    pub log_path: String,
    pub split_path: String,
    pub chan_len: usize,
    pub log_size: usize,
    pub keep_file_num: i64,
}

/// init config
pub fn init_config(path: PathBuf) -> Result<Setting> {
    let mut conf_str = String::default();
    let mut file = File::open(path)?;
    file.read_to_string(&mut conf_str)?;
    let setting =
        toml::from_str::<Setting>(&conf_str).expect("Parsing the configuration file failed");
    Ok(setting)
}
