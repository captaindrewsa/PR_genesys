#![allow(warnings)]
mod database;
mod parsing;
mod smthng;
mod utils;

use crate::database::db::{workingWithProjects, TheOneDatabase};

use bson::oid::ObjectId;
use smthng::loger;
use tokio;

#[tokio::main]
async fn main() {
    let mut database = TheOneDatabase {
        database_kegg: mongodb::Client::with_uri_str("mongo://127.0.0.1:27017")
            .await
            .unwrap()
            .database("kegobb"),
        database_prj: mongodb::Client::with_uri_str("mongo://127.0.0.1:27017")
            .await
            .unwrap()
            .database("Projects"),
        collection_prj: "Projects".to_string(),
    };

    let prj = database.create_project("Test_prj").await.unwrap();

    database.create_comp(prj, "Compartment3123").await;
    // database.create_daughter_comp(prj, "father_comp", "daug_comp").unwrap();
    // database
    //     .create_father_comp(prj, "father_comp", "daughter_comp")
    //     .unwrap();
    // database
    //     .update_kegg_comp(prj, "Compartment", "Entry")
    //     .unwrap();
}
