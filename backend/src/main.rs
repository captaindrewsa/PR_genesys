#![allow(warnings)]
mod database;
mod parsing;
mod smthng;

use backend::database::db::{workingWithProjects, Kegg_database, Project_database};

use bson::oid::ObjectId;
use smthng::loger;
use tokio;

#[tokio::main]
async fn main() {
    let database = Project_database { database: 
        mongodb::Client::with_uri_str("127.0.0.1:27017").await.unwrap().database("Project") };

    let prj = database.create_project("Test_prj").await;

    // database.create_comp(prj, "Compartment").unwrap();
    // database
    //     .create_daughter_comp(prj, "father_comp", "daug_comp")
    //     .unwrap();
    // database
    //     .create_father_comp(prj, "father_comp", "daughter_comp")
    //     .unwrap();
    // database
    //     .update_kegg_comp(prj, "Compartment", "Entry")
    //     .unwrap();
}
