#![allow(warnings)]
mod database;
mod parsing;
mod smthng;
mod utils;

use crate::database::db::{workingWithProjects, TheOneDatabase};

use bson::oid::ObjectId;
use database::db::workingWithKegg;
use parsing::{IParser, Parser};
use smthng::loger;
use tokio;

#[tokio::main]
async fn main() {
    let mut database = TheOneDatabase {
        database_kegg: mongodb::Client::with_uri_str("mongodb://127.0.0.1:27017")
            .await
            .unwrap()
            .database("kegobb"),
        database_prj: mongodb::Client::with_uri_str("mongodb://127.0.0.1:27017")
            .await
            .unwrap()
            .database("Projects"),
        collection_prj: "Projects".to_string(),
    };

    // Parser::get_kegg("https://www.kegg.jp/entry/2.2.1.5").await;
    database.add_kegg(Parser::get_kegg("https://www.kegg.jp/entry/2.2.1.5").await).await;


    let prj = database.create_project("Test_insert").await.unwrap();

    database.create_comp(prj, "Comp_insert").await;
    // database.create_daughter_comp(prj, "father_comp", "daug_comp").unwrap();
    // database
    //     .create_father_comp(prj, "father_comp", "daughter_comp")
    //     .unwrap();
    database
        .update_kegg_comp(prj, "Comp_insert", "5315")
        .await
        .unwrap();
}
