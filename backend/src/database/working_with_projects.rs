#![allow(warnings)]
use std::str::FromStr;

use bson::oid::ObjectId;

use crate::databaseQuery;

use super::db::{workingWithProjects, Project_database};

impl workingWithProjects for Project_database {
    async fn create_project(
        &mut self,
        prj_name: &str,
    ) -> Result<bson::oid::ObjectId, databaseQuery> {
        
        Ok(ObjectId::from_str("123").unwrap())
    }

    async fn create_comp(
        prj: bson::oid::ObjectId,
        comp_name: &str)-> Option<bson::oid::ObjectId> {
        todo!()
    }

    async fn create_daughter_comp(
        prj: bson::oid::ObjectId,
        father_comp: &str,
        daughter_comp: &str) -> Option<bson::oid::ObjectId> {
        todo!()
    }

    async fn create_father_comp(
        prj: bson::oid::ObjectId,
        father_comp: &str,
        daughter_comp: &str) -> Option<bson::oid::ObjectId> {
        todo!()
    }

    async fn update_kegg_comp(
        prj: bson::oid::ObjectId, 
        comp_name: &str,
        entry_name: &str)-> Option<bson::oid::ObjectId> {
        todo!()
    }
}
