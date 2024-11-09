use std::{fs::File, time::UNIX_EPOCH};

use lazy_static::lazy_static;
use simplelog::{CombinedLogger, ConfigBuilder, TermLogger, WriteLogger};



pub fn loger(){
    let log_config = ConfigBuilder::new()
    .set_level_color(log::Level::Info, Some(simplelog::Color::Green))
    .add_filter_allow_str("backend")
    .build();

    CombinedLogger::init(vec![
        TermLogger::new(
            log::LevelFilter::Info,
            log_config.clone(),
            simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
        ) ,
        WriteLogger::new(
            log::LevelFilter::Trace,
            log_config.clone(),
            File::create(format!("/home/captaindrewsa/Programming/Rust/PR_genesys/backend/logs/log_{}.log",
                std::time::SystemTime::now().duration_since(UNIX_EPOCH).expect(" calculate DateTime for log is error").as_secs()
            )).unwrap(),
    ),
])
.unwrap();
}