#![allow(warnings)]
mod database;
mod parsing;

use parsing::{
    schemas::{kegg_schemas, CDS},
    IParser, Parser,
};

use database::db::{self, workingWithKegg, Kegg_database};
use mongodb::{
    self,
    bson::{self, doc},
};
use tokio;

#[tokio::main]
async fn main() {

    let url_kegg = vec![
        // "https://www.genome.jp/entry/1.1.1.27",
        // "https://www.genome.jp/entry/1.14.14.51",
        // "https://www.genome.jp/entry/1.1.3.8",
        // "https://www.genome.jp/entry/4.1.3.3",
        // "https://www.genome.jp/entry/4.1.3.38",
        "https://www.genome.jp/entry/7.5.2.3",
        "https://www.genome.jp/entry/7.6.2.1",
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
            Ok(_) => println!("Добавили"),
            Err(_) => println!("Ошибили"),
        };
    }

}
