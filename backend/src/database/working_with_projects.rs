#![allow(warnings)]
use std::str::FromStr;

use bson::{doc, oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};
use regex;

use crate::utils::{databaseQuery, kegg_schemas, Compartment, Project_default};

use super::db::{workingWithProjects, TheOneDatabase};

trait parse_Kegg_from_query_to_db {
    async fn get_bson_from_entry(&mut self, entry: &str) -> Option<kegg_schemas>;
}

impl workingWithProjects for TheOneDatabase {
    async fn create_project(
        &mut self,
        prj_name: &str,
    ) -> Result<bson::oid::ObjectId, databaseQuery> {
        let mut tmp = Project_default::default();
        tmp.Name = prj_name.to_owned();
        let obj_id: ObjectId = {
            if let Some(obj) = self
                .database_prj
                .collection::<Bson>(&self.collection_prj)
                .find_one(doc! {"Name": prj_name})
                .await
                .unwrap()
            {
                println!("Такой документ был найден, но вернут до стандартного");

                self.database_prj
                    .collection(&self.collection_prj)
                    .find_one_and_replace(doc! {"Name": prj_name}, bson::to_bson(&tmp).unwrap())
                    .upsert(true)
                    .await;

                let tmp = obj
                    .as_document()
                    .unwrap()
                    .get("_id")
                    .unwrap()
                    .clone()
                    .as_object_id()
                    .unwrap();

                tmp
            } else {
                let tmp = self
                    .database_prj
                    .collection(&self.collection_prj)
                    .insert_one(bson::to_bson(&tmp).unwrap())
                    .await
                    .unwrap()
                    .inserted_id
                    .as_object_id()
                    .unwrap();

                tmp
            }
        };

        Ok(obj_id)
    }

    async fn create_comp(
        &mut self,
        prj: bson::oid::ObjectId,
        comp_name: &str,
    ) -> Option<bson::oid::ObjectId> {
        let mut tmp = Compartment::default();
        tmp.Name = comp_name.to_string();

        if let Some(obj) = self
            .database_prj
            .collection::<Bson>("Projects")
            .find_one(doc! {"_id": prj})
            .await
            .unwrap()
        {
            self.database_prj
                .collection::<Bson>(&self.collection_prj)
                .update_one(
                    doc! {"_id": prj, "Compartments.Name":{"$ne":comp_name}
                    },
                    doc! {"$push": {
                        "Compartments": bson::to_bson(&tmp).unwrap()
                    }},
                )
                .upsert(true)
                .await;

            Some(prj)
        } else {
            None
        }
    }

    async fn create_daughter_comp(
        prj: bson::oid::ObjectId,
        father_comp: &str,
        daughter_comp: &str,
    ) -> Option<bson::oid::ObjectId> {
        todo!()
    }

    async fn create_father_comp(
        prj: bson::oid::ObjectId,
        father_comp: &str,
        daughter_comp: &str,
    ) -> Option<bson::oid::ObjectId> {
        todo!()
    }

    async fn update_kegg_comp(
        &mut self,
        prj: bson::oid::ObjectId,
        comp_name: &str,
        entry_name: &str,
    ) -> Option<bson::oid::ObjectId> {
        let kegg_ob = match self.get_bson_from_entry(entry_name).await.unwrap() {
            
            kegg_schemas::CDS(bson) => bson,
            kegg_schemas::Enzyme(bson) => bson,
            kegg_schemas::Reaction(bson) => bson,
            kegg_schemas::Compound(bson) => bson,
            kegg_schemas::Error(err) => bson::to_bson(&err).unwrap(),
        };

        // println!("\n{}\n", kegg_ob.as_document().unwrap());

        self.database_prj
            .collection::<Bson>(&self.collection_prj)
            .update_one(
                doc! {"_id": prj  , "Compartments.Name":comp_name},
                doc! {"$addToSet":{
                    "Compartments.$[elem].Objects": kegg_ob
                }},
            )
            .array_filters([doc! {"elem.Name":comp_name}])
            .await
            .unwrap();

        Some(prj)
    }
}

impl parse_Kegg_from_query_to_db for TheOneDatabase {
    async fn get_bson_from_entry(&mut self, entry: &str) -> Option<kegg_schemas> {
        
        let re_reaction = regex::Regex::new(r"^R[0-9]{5}").unwrap();
        let re_enzyme = regex::Regex::new(r"^[0-9]\.").unwrap();
        let re_cds = regex::Regex::new(r"^[0-9]").unwrap();
        let re_compound = regex::Regex::new(r"^C[0-9]{5}").unwrap();
        
        
        let schema = if re_enzyme.is_match(entry) {
            let kegg_bson = self
                .database_kegg
                .collection::<Bson>("Enzyme")
                .find_one(doc! {"Entry": bson::to_bson(entry).unwrap()})
                .projection(doc! {"Type":true, "Entry": true})
                .await
                .unwrap();

            /* 
            Необходимо сделать докачку в случае отсутствия в бд
             */
            kegg_schemas::Enzyme(kegg_bson.unwrap())
        } else if re_reaction.is_match(entry) {
            let kegg_bson = self
                .database_kegg
                .collection::<Bson>("Reaction")
                .find_one(doc! {"Entry": entry})
                .projection(doc! {"Type":true, "Entry": true})
                .await
                .unwrap();
            kegg_schemas::Reaction(kegg_bson.unwrap())
        } else if re_cds.is_match(entry) {
            let kegg_bson = self
                .database_kegg
                .collection::<Bson>("CDS")
                .find_one(doc! {"Entry": entry})
                .projection(doc! {"Type":true, "Entry": true})
                .await
                .unwrap();
            kegg_schemas::CDS(kegg_bson.unwrap())
        } else if re_compound.is_match(entry) {
            let kegg_bson = self
                .database_kegg
                .collection::<Bson>("Compound")
                .find_one(doc! {"Entry": entry})
                .projection(doc! {"Type":true, "Entry": true})
                .await
                .unwrap();
            kegg_schemas::Compound(kegg_bson.unwrap())
        } else {
            kegg_schemas::Error("None".to_string())
        };

        Some(schema)
    }
}
