#![allow(warnings)]
mod parsing;
use parsing::{schemas::{kegg_schemas, CDS}, Parser};

use json;
use serde_json;

use tokio;

use mongodb::{self, bson::{self, doc}};


#[tokio::main]
async fn main() {
    
    let url_kegg = "https://www.genome.jp/entry/hsa:5313";
    let mut result_doc = Parser::get_kegg(&url_kegg).await;

    println!("{:?}", result_doc);
    
    
    let collection = "NewKegobb";
    let uri = "mongodb://127.0.0.1:27017";
    let client = mongodb::Client::with_uri_str(uri).await.unwrap();
    let dbs = client.database("kegobb");
    
    // dbs.collection::<parsing::schemas::CDS>(&collection).insert_one(result_des).await.unwrap();
    
    
}

