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
    let url_kegg = vec![
        "https://www.genome.jp/entry/2.7.1.40",
        "https://www.genome.jp/entry/R00200",
        "https://www.genome.jp/entry/C00002",
        "https://www.genome.jp/entry/hsa:5315"];
        
        let mut asdasd = WorkingWithKegg::Kegg_database {
            database: mongodb::Client::with_uri_str("mongodb://127.0.0.1:27017")
                .await
                .unwrap()
                .database("kegobb"),
            kegg_collection: "NewKegobb",
        };
    for elem in url_kegg{
        let result_doc = Parser::get_kegg(elem).await;
        match asdasd.add_kegg(result_doc).await {
            Ok(_) => println!("Добавить получилось"),
            Err(_) => println!("Ошибка"),
        }

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
        async fn find_kegg_by_id(&mut self, id_string: String) -> Option<kegg_schemas>;
        async fn add_kegg(&mut self, kegg_sh: kegg_schemas) -> Result<databaseQuery, databaseQuery>;
        fn update_kegg_by_id(
            &mut self,
            id_string: String,
            new_data: kegg_schemas,
        ) -> Result<databaseQuery, databaseQuery>;
    }

    impl workingWithKegg for Kegg_database {
        async fn find_kegg_by_id(&mut self, id_string: String) -> Option<kegg_schemas> {
            // let tmp = self.database.collection(self.kegg_collection).find_one(doc! {"_id": id_string}).await.unwrap();

            None
        }

        async fn add_kegg(&mut self, kegg_sh: kegg_schemas) -> Result<databaseQuery, databaseQuery> {
            match kegg_sh {
                
                kegg_schemas::CDS(var) => {
                    
                    let var_des: schemas::CDS = bson::from_bson(var).unwrap();

                    let var_entry = var_des.Entry.clone();

                    self.database
                        .collection(&self.kegg_collection)
                        .find_one_and_replace(
                            doc! {"Entry": var_entry}, 
                            bson::to_bson(&var_des).unwrap())
                            .upsert(true)
                        .await
                        .unwrap();
                }
                kegg_schemas::Enzyme(var) => {
                    let var_des: schemas::Enzyme = bson::from_bson(var).unwrap();
                    let var_entry = var_des.Entry.clone();


                    self.database
                        .collection(&self.kegg_collection)
                        .find_one_and_replace(
                            doc! {"Entry": var_entry}, 
                            bson::to_bson(&var_des).unwrap())
                            .upsert(true)
                        .await
                        .unwrap();
                }
                kegg_schemas::Reaction(var) => {
                    // println!("=====\n{:?}\n======", var.as_document().unwrap().to_string());
                    let var_des: schemas::Reaction = bson::from_bson(var).unwrap();
                    let var_entry = var_des.Entry.clone();

   
                    self.database
                        .collection(&self.kegg_collection)
                        .find_one_and_replace(
                            doc! {"Entry": var_entry}, 
                            bson::to_bson(&var_des).unwrap())
                            .upsert(true)
                        .await
                        .unwrap();
                }
                kegg_schemas::Compound(var) => {
                    let var_des: schemas::Compound = bson::from_bson(var).unwrap();
                    let var_entry = var_des.Entry.clone();


                    self.database
                        .collection(&self.kegg_collection)
                        .find_one_and_replace(
                            doc! {"Entry": var_entry}, 
                            bson::to_bson(&var_des).unwrap())
                            .upsert(true)
                        .await
                        .unwrap();
                }
                kegg_schemas::Error(var) => {
                    return Err(databaseQuery::Error("Incorrect adding kegg".to_string()))
                }
            }
            fn get_entry(keg_sh: kegg_schemas)-> String{
                match keg_sh {
                    kegg_schemas::CDS(bson) => todo!(),
                    kegg_schemas::Enzyme(bson) => todo!(),
                    kegg_schemas::Reaction(bson) => todo!(),
                    kegg_schemas::Compound(bson) => todo!(),
                    kegg_schemas::Error(_) => todo!(),
                }
            };

            Ok(databaseQuery::Ok)



        }

        fn update_kegg_by_id(
            &mut self,
            id_string: String,
            new_data: kegg_schemas,
        ) -> Result<databaseQuery, databaseQuery> {
            Ok(databaseQuery::Ok)
        }
    }
}

enum databaseQuery {
    Ok,
    Error(String),
}
