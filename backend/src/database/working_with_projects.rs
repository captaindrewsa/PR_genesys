#![allow(warnings)]
use std::str::FromStr;

use bson::{doc, oid::ObjectId, Bson};
use serde::{Deserialize, Serialize};

use crate::utils::databaseQuery;

use super::db::{workingWithProjects, Project_database};

impl workingWithProjects for Project_database {
    async fn create_project(
        &mut self,
        prj_name: &str,
    ) -> Result<bson::oid::ObjectId, databaseQuery> {
        #[derive(Serialize, Deserialize, Default)]
        struct Compartment {
            Objects: Vec<bson::Bson>,
        }

        #[derive(Serialize, Deserialize, Default)]
        struct Project_default {
            Name: String,
            Compartments: Vec<Compartment>,
        }

        let mut tmp = Project_default::default();
        tmp.Name = prj_name.to_owned();
        let obj_id: ObjectId = {
            if let Some(obj) = self
                .database
                .collection::<Bson>("Projects")
                .find_one(doc! {"Name": prj_name})
                .await
                .unwrap()
            {
                println!("Такой документ был найден, но вернут до стандартного");

                self.database
                    .collection("Projects")
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
                    .database
                    .collection("Projects")
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

    async fn create_comp(prj: bson::oid::ObjectId, comp_name: &str) -> Option<bson::oid::ObjectId> {
        todo!()
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
        prj: bson::oid::ObjectId,
        comp_name: &str,
        entry_name: &str,
    ) -> Option<bson::oid::ObjectId> {
        todo!()
    }
}
