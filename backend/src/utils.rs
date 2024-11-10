#![allow(warnings)]

use bson::oid::ObjectId;
use bson::Bson;
use serde::Deserialize;
use serde::Serialize;

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

#[derive(Serialize, Deserialize, Default)]
pub struct Compartment {
    _id: ObjectId,
    pub Name: String,
    pub Objects: Vec<bson::Bson>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Project_default {
    _id: ObjectId,
    pub Name: String,
    pub Compartments: Vec<Compartment>,
}
