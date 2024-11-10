use bson::doc;
use log::{info, trace};

use crate::utils::{databaseQuery, kegg_schemas};

use super::db::{workingWithKegg, Kegg_database};

impl workingWithKegg for Kegg_database {
    async fn add_kegg(&mut self, kegg_sh: kegg_schemas) -> Result<databaseQuery, databaseQuery> {
        match kegg_sh {
            kegg_schemas::CDS(var) => {
                let var_des = var.as_document().unwrap();
                let var_entry = var_des.get_str("Entry").unwrap();

                self.database
                    .collection("CDS")
                    .find_one_and_replace(
                        doc! {"Entry": var_entry},
                        bson::to_bson(&var_des).unwrap(),
                    )
                    .upsert(true)
                    .await
                    .unwrap();
            }
            kegg_schemas::Enzyme(var) => {
                let var_des = var.as_document().unwrap();
                let var_entry = var_des.get_str("Entry").unwrap();

                self.database
                    .collection("Enzyme")
                    .find_one_and_replace(
                        doc! {"Entry": var_entry},
                        bson::to_bson(&var_des).unwrap(),
                    )
                    .upsert(true)
                    .await
                    .unwrap();
            }
            kegg_schemas::Reaction(var) => {
                let var_des = var.as_document().unwrap();
                let var_entry = var_des.get_str("Entry").unwrap();

                self.database
                    .collection("Reaction")
                    .find_one_and_replace(
                        doc! {"Entry": var_entry},
                        bson::to_bson(&var_des).unwrap(),
                    )
                    .upsert(true)
                    .await
                    .unwrap();
            }
            kegg_schemas::Compound(var) => {
                let var_des = var.as_document().unwrap();
                let var_entry = var_des.get_str("Entry").unwrap();

                self.database
                    .collection("Compound")
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
        return Ok(databaseQuery::Ok);
    }
}
