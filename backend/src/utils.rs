#![allow(warnings)]

use bson::Bson;

use crate::database;
use crate::parsing;

#[derive(Debug)]
pub enum kegg_schemas {
    CDS(Bson),
    Enzyme(Bson),
    Reaction(Bson),
    Compound(Bson),
    Error(String),
}

#[derive(Debug)]
pub enum databaseQuery {
    Ok,
    Error(String),
}
