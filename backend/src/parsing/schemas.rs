use std::collections::HashMap;

use bson::Bson;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum kegg_schemas {
    CDS(Bson),
    Enzyme(Bson),
    Reaction(Bson),
    Compound(Bson),
    Error(String),
}

#[derive(Serialize, Deserialize, Debug)]

pub struct definition {
    pub Substrate: Vec<String>,
    pub Product: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct equation {
    pub Substrate: Vec<String>,
    pub Product: Vec<String>,
}

pub enum databaseQuery {
    Ok,
    Error(String),
}
