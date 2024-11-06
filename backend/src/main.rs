#![allow(warnings)]
mod parsing;
use parsing::{
    schemas::{kegg_schemas, CDS},
    IParser, Parser,
};
mod database;
use database::db::{self, workingWithKegg, Kegg_database};

use json;
use serde_json;

use tokio;

use mongodb::{
    self,
    bson::{self, doc},
};

#[tokio::main]
async fn main() {
    let url_kegg = vec![
        "https://www.genome.jp/entry/2.7.1.40",
        "https://www.genome.jp/entry/R00200",
        "https://www.genome.jp/entry/C00002",
        "https://www.genome.jp/entry/hsa:5315",
    ];

    let mut asdasd = Kegg_database {
        database: mongodb::Client::with_uri_str("mongodb://127.0.0.1:27017")
            .await
            .unwrap()
            .database("kegobb"),
        kegg_collection: "NewKegobb",
    };

    for elem in url_kegg {
        let mut dabas = asdasd.clone();
        let tmp_doc = Parser::get_kegg(elem).await;
        match dabas.add_kegg(tmp_doc).await {
            Ok(_) => println!("Добавили"),
            Err(_) => println!("Наошибили"),
        };
    }
}
