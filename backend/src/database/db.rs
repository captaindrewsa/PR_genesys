use std::borrow::Borrow;

use mongodb::bson::doc;

use crate::parsing::{
    self,
    schemas::{self, databaseQuery, kegg_schemas, Compound, Enzyme, Reaction, CDS},
};

#[derive(Clone)]
pub struct Kegg_database {
    pub database: mongodb::Database,
    pub kegg_collection: &'static str,
}

pub trait workingWithKegg {
    async fn add_kegg(
        &mut self,
        kegg_sh: kegg_schemas,
    ) -> Result<schemas::databaseQuery, schemas::databaseQuery>;
}
