#![allow(warnings)]
use std::str;

use bson::oid::ObjectId;

use crate::utils::{databaseQuery, kegg_schemas};

#[derive(Clone)]
pub struct TheOneDatabase {
    pub database_kegg: mongodb::Database,
    pub database_prj: mongodb::Database,
    pub collection_prj: String,
}

pub trait workingWithKegg {
    async fn add_kegg(&mut self, kegg_sh: kegg_schemas) -> Result<databaseQuery, databaseQuery>;
}

pub trait workingWithProjects {
    async fn create_project(&mut self, prj_name: &str) -> Result<ObjectId, databaseQuery>;
    async fn create_comp(&mut self, prj: ObjectId, comp_name: &str) -> Option<ObjectId>;
    async fn create_daughter_comp(
        prj: ObjectId,
        father_comp: &str,
        daughter_comp: &str,
    ) -> Option<ObjectId>;
    async fn create_father_comp(
        prj: ObjectId,
        father_comp: &str,
        daughter_comp: &str,
    ) -> Option<ObjectId>;
    async fn update_kegg_comp(
        &mut self,
        prj: ObjectId,
        comp_name: &str,
        entry_name: &str,
    ) -> Option<ObjectId>;
}
