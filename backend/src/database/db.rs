use log;
use std::borrow::Borrow;

use mongodb::bson::doc;

use crate::parsing::{
    self,
    schemas::{self, databaseQuery, kegg_schemas},
};

#[derive(Clone)]
pub struct Kegg_database {
    pub database: mongodb::Database,
}

#[derive(Clone)]
pub struct Project_database {
    pub database: mongodb::Database,
}



pub trait workingWithKegg {
    async fn add_kegg(
        &mut self,
        kegg_sh: kegg_schemas,
    ) -> Result<schemas::databaseQuery, schemas::databaseQuery>;
}

pub trait workingWithProjects {
    async fn create_project(&mut self, prj_name: &str)-> Result<schemas::databaseQuery,schemas::databaseQuery>;
    async fn project_is_exist();

}