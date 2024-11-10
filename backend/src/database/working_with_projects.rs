#![allow(warnings)]
use std::str::FromStr;

use bson::{doc, oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

use crate::utils::{databaseQuery, Compartment, Project_default};

use super::db::{workingWithProjects, TheOneDatabase};

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
        self.database_prj
            .collection::<Bson>(&self.collection_prj)
            .find_one_and_update(doc! {"_id": prj, "Compartments.Name":comp_name}, doc! {})
            .await;

        todo!()
    }
}
