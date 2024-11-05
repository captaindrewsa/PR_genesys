use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Enzyme{
    Entry: String,
    Type: String,
    Name: Vec<String>,
    Reaction_KEGG: Vec<String>,
    Substrate: Vec<String>,
    Product: Vec<String>,
    Pathway: Vec<String>,
    Genes: std::collections::HashMap<String, Vec<String>>,
    Reaction_IUBMB: Vec<String>
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Reaction{
    Entry: String,
    Name: Vec<String>,
    Definition: std::collections::HashMap<String, Vec<String>>,
    Equation: std::collections::HashMap<String, Vec<String>>,
    Enzyme: Vec<String>,
    Type: String,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct Compound{
    Entry: String,
    Name: Vec<String>,
    Formula: String,
    Reaction: Vec<String>,
    Enzyme: Vec<String>,
    Type: String,
}
#[derive(Serialize, Deserialize, Debug)]

pub struct CDS{
    Entry: String,
    Symbol: Vec<String>,
    Name: Vec<String>,
    Organism: Vec<String>,
    Pathway: Vec<Vec<String>>,
    Module: Vec<Vec<String>>,
    AA_seq: String,
    NT_seq: String,
    Type: String,
}