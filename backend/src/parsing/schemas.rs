use std::collections::HashMap;

use bson::Bson;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Enzyme {
    pub Entry: String,
    pub Type: String,
    pub Name: Vec<String>,
    pub Reaction_KEGG: Vec<String>,
    pub Substrate: Vec<String>,
    pub Product: Vec<String>,
    pub Pathway: Vec<Vec<String>>,
    pub Genes: std::collections::HashMap<String, Vec<String>>,
    pub Reaction_IUBMB: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Reaction {
    pub Entry: String,
    pub Type: String,
    pub Name: Vec<String>,
    pub Definition: std::collections::HashMap<String, Vec<String>>,
    pub Equation: std::collections::HashMap<String, Vec<String>>,
    pub Enzyme: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Compound {
    pub Entry: String,
    pub Type: String,
    pub Name: Vec<String>,
    pub Formula: String,
    pub Reaction: Vec<String>,
    pub Enzyme: Vec<String>,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct CDS {
    pub Entry: String,
    pub Type: String,
    pub Name: Vec<String>,
    pub Symbol: Vec<String>,
    pub Organism: Vec<String>,
    pub Pathway: Vec<Vec<String>>,
    pub Module: Vec<Vec<String>>,
    pub AA_seq: String,
    pub NT_seq: String,
}

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
