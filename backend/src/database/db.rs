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

pub trait workingWithKegg {
    async fn add_kegg(
        &mut self,
        kegg_sh: kegg_schemas,
    ) -> Result<schemas::databaseQuery, schemas::databaseQuery>;
}
