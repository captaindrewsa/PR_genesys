use bson::doc;

use crate::parsing::schemas::{self, databaseQuery, kegg_schemas};

use super::db::{workingWithKegg, Kegg_database};

impl workingWithKegg for Kegg_database {
    async fn add_kegg(
        &mut self,
        kegg_sh: kegg_schemas,
    ) -> Result<schemas::databaseQuery, schemas::databaseQuery> {
        match kegg_sh {
            kegg_schemas::CDS(var) => {
                let var_des: schemas::CDS = bson::from_bson(var).unwrap();

                let var_entry = var_des.Entry.clone();

                self.database
                    .collection(&self.kegg_collection)
                    .find_one_and_replace(
                        doc! {"Entry": var_entry},
                        bson::to_bson(&var_des).unwrap(),
                    )
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
                        bson::to_bson(&var_des).unwrap(),
                    )
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
                        bson::to_bson(&var_des).unwrap(),
                    )
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
                        bson::to_bson(&var_des).unwrap(),
                    )
                    .upsert(true)
                    .await
                    .unwrap();
            }
            kegg_schemas::Error(var) => {
                return Err(databaseQuery::Error("Incorrect adding kegg".to_string()))
            }
        }
        return Ok(schemas::databaseQuery::Ok);
    }
}
