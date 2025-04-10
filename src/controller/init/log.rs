use crate::controller::init::config::Log;
use anyhow::Result;
use fast_log::consts::LogSize;
use fast_log::plugin::file_split::RollingType;
use fast_log::plugin::packer::LogPacker;
use fast_log::Config;
use log::LevelFilter;

pub(crate) fn init_log(conf: &Log) -> Result<()> {
    let mut conf_setting = Config::new()
        .file(&conf.log_path)
        .chan_len(Some(conf.chan_len))
        .level(LevelFilter::Info)
        .file_split(
            &conf.split_path,
            LogSize::MB(conf.log_size),
            RollingType::KeepNum(conf.keep_file_num),
            LogPacker {},
        );
    if conf.debug {
        conf_setting = conf_setting.console();
    }
    fast_log::init(conf_setting)?;
    Ok(())
}
