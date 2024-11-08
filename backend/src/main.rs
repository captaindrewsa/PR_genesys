#![allow(warnings)]
mod database;
mod parsing;

use std::{env, fs::File};

use parsing::{
    schemas::{kegg_schemas, CDS},
    IParser, Parser,
};

use database::db::{self, workingWithKegg, Kegg_database};
use log::{error, info, log_enabled, trace, warn};

use mongodb::{
    self,
    bson::{self, doc},
};
use simplelog::{CombinedLogger, Config, ConfigBuilder, TermLogger, WriteLogger};
use tokio;

#[tokio::main]
async fn main() {

    let log_config = ConfigBuilder::new()
    .set_level_color(log::Level::Info, Some(simplelog::Color::Green))
    .add_filter_allow_str("backend").build();
    
    CombinedLogger::init(vec![
        TermLogger::new(log::LevelFilter::Info, log_config.clone(), simplelog::TerminalMode::Mixed, simplelog::ColorChoice::Auto),
        WriteLogger::new(log::LevelFilter::Trace, log_config.clone(), File::create("my_log_test.log").unwrap())
    ]).unwrap();

    let url_kegg = vec![
        "https://www.genome.jp/entry/7.2.2.13",
        // "https://www.genome.jp/entry/1.14.14.51",
        // "https://www.genome.jp/entry/1.1.3.8",
        // "https://www.genome.jp/entry/4.1.3.3",
        // "https://www.genome.jp/entry/4.1.3.38",
        // "https://www.genome.jp/entry/7.5.2.3",
        // "https://www.genome.jp/entry/7.6.2.1",
    ];

    let mut dabas = Kegg_database {
        database: mongodb::Client::with_uri_str("mongodb://127.0.0.1:27017")
            .await
            .unwrap()
            .database("kegobb"),
    };

    for elem in url_kegg {
        let tmp_doc = Parser::get_kegg(elem).await;
        match dabas.add_kegg(tmp_doc).await {
            Ok(_) => {
                continue;
            }
            Err(_) => {
                continue;
            }
        };
    }
}
