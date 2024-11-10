use bson::Bson;

pub mod database;
pub mod parsing;
pub mod smthng;


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