#![allow(warnings)]
mod parsing;
use parsing::{
    schemas::{kegg_schemas, CDS},
    IParser, Parser,
};

use json;
use serde_json;

use tokio;

use mongodb::{
    self,
    bson::{self, doc},
};
use WorkingWithKegg::workingWithKegg;

#[tokio::main]
async fn main() {
    let url_kegg = "https://www.genome.jp/entry/3.6.1.5";
    let result_doc = Parser::get_kegg(&url_kegg).await;




    // match result_doc {
    // kegg_schemas::CDS(var) => println!("{:?}", var),
    // kegg_schemas::Enzyme(var) => println!("{:?}", var),
    // kegg_schemas::Reaction(var) => println!("{:?}", var),
    // kegg_schemas::Compound(var) => println!("{:?}", var),
    // kegg_schemas::Error(var) => println!("{:?}", var),
    //     }

    // let collection = "NewKegobb";
    // let uri = "mongodb://127.0.0.1:27017";

    let mut asdasd = WorkingWithKegg::Kegg_database {
        database: mongodb::Client::with_uri_str("mongodb://127.0.0.1:27017")
            .await
            .unwrap()
            .database("kegobb"),
        kegg_collection: "NewKegobb",
    };

    match asdasd.add_kegg(result_doc).await {
        Ok(_) => println!("Добавить получилось"),
        Err(_) => println!("Ошибка"),
    }
}
pub mod WorkingWithKegg {
    use std::borrow::Borrow;

    use mongodb::bson::doc;

    use crate::{
        databaseQuery,
        parsing::{
            self,
            schemas::{self, kegg_schemas, Compound, Enzyme, Reaction, CDS},
        },
    };

    pub struct Kegg_database {
        pub database: mongodb::Database,
        pub kegg_collection: &'static str,
    }

    pub trait workingWithKegg {
        fn find_kegg_by_id(&mut self, id_string: String) -> Option<kegg_schemas>;
        async fn add_kegg(&mut self, kegg_sh: kegg_schemas) -> Result<databaseQuery, databaseQuery>;
        fn update_kegg_by_id(
            &mut self,
            id_string: String,
            new_data: kegg_schemas,
        ) -> Result<databaseQuery, databaseQuery>;
    }

    impl workingWithKegg for Kegg_database {
        fn find_kegg_by_id(&mut self, id_string: String) -> Option<kegg_schemas> {
            // let tmp = self.database.collection(self.kegg_collection).find_one(doc! {"_id": id_string}).

            None
        }

        async fn add_kegg(&mut self, kegg_sh: kegg_schemas) -> Result<databaseQuery, databaseQuery> {
            
            match kegg_sh {
                
                kegg_schemas::CDS(var) => {
                    
                    let var_des: schemas::CDS = bson::from_bson(var).unwrap();
                    
                    self.database
                        .collection(&self.kegg_collection)
                        .insert_one(var_des)
                        .await
                        .unwrap();
                }
                kegg_schemas::Enzyme(var) => {
                    
                    // println!("{:?}", var.as_document());

                    let var_des: schemas::Enzyme = bson::from_bson(var).unwrap();
                    
                    self.database
                        .collection(&self.kegg_collection)
                        .insert_one(var_des)
                        .await
                        .unwrap();
                }
                kegg_schemas::Reaction(var) => {
                    let var_des: schemas::Reaction = bson::from_bson(var).unwrap();
   
                    self.database
                        .collection(&self.kegg_collection)
                        .insert_one(var_des)
                        .await
                        .unwrap();
                }
                kegg_schemas::Compound(var) => {
                    let var_des: schemas::Compound = bson::from_bson(var).unwrap();
                    self.database
                        .collection(&self.kegg_collection)
                        .insert_one(var_des)
                        .await
                        .unwrap();
                }
                kegg_schemas::Error(var) => {
                    return Err(databaseQuery::Error("Incorrect adding kegg".to_string()))
                }
            }

            Ok(databaseQuery::Ok)
        }

        fn update_kegg_by_id(
            &mut self,
            id_string: String,
            new_data: kegg_schemas,
        ) -> Result<databaseQuery, databaseQuery> {
            todo!()
        }
    }
}

enum databaseQuery {
    Ok,
    Error(String),
}
