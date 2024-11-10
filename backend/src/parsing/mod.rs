mod parsers;
pub mod schemas;
use bson::{doc, Bson};
use log::{info, trace};
use parsers::*;

use json::{self as other_json, object};
use regex;
use reqwest;
use schemas::{kegg_schemas};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json::{self, json};
use std::{borrow::Borrow, collections::HashMap, str::FromStr};
use tokio;

pub struct Parser {}

pub trait IParser {
    async fn parse_kegg_to_string(kegg_url: &str) -> Option<Vec<Bson>>;
    async fn vec_bson_to_kegg_schemas(vec_bson: Vec<Bson>) -> kegg_schemas;
    async fn get_kegg(url: &str) -> kegg_schemas;
}

impl IParser for Parser {
    async fn parse_kegg_to_string(kegg_url: &str) -> Option<Vec<Bson>> {
        let mut otp_string = Vec::new();

        let resp = reqwest::get(kegg_url).await.unwrap().text().await.unwrap();
        let fragment = Html::parse_document(&resp);

        let main_table_selector = Selector::parse("table.w2").unwrap();
        let tr_table_selector = Selector::parse("tr").unwrap();
        let th_selector = Selector::parse("th").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        let span_selector = Selector::parse("span").unwrap();

        let main_table = fragment.select(&main_table_selector).next().unwrap();

        for tr in main_table.select(&tr_table_selector) {
            /* Здесь нужно устроить поиск внутри tr с распределением по th и td */

            /* Ниже рабочий поиск по th */

            for th in tr.select(&th_selector) {
                /* Здесь нужно вычленить span для определения названия строчки*/
                for span in th.select(&span_selector) {
                    if let Some(name_of_row) = span.text().collect::<Vec<&str>>().first().cloned() {
                        for td in tr.select(&td_selector) {
                            // println!("=====\n{}\n==========", td.html());

                            match name_of_row {
                                "Entry" => {
                                    if let Some(data) = entry_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Name" => {
                                    if let Some(data) = name_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Formula" => {
                                    if let Some(data) = formula_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Exact mass" => {
                                    if let Some(data) = exact_mass_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Mol weight" => {
                                    if let Some(data) = mol_weight_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                // "Structure" => todo!("Сделать загрузку Mol File"),
                                "Reaction" => {
                                    if let Some(data) = reaction_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Enzyme" => {
                                    if let Some(data) = enzyme_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Pathway" => {
                                    if let Some(data) = pathway_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Module" => {
                                    if let Some(data) = module_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Definition" => {
                                    if let Some(data) = definition_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Equation" => {
                                    if let Some(data) = equation_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Reaction(IUBMB)" => {
                                    if let Some(data) = reaction_iubmb_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Reaction(KEGG)" => {
                                    if let Some(data) = reaction_kegg_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Substrate" => {
                                    if let Some(data) = substrate_kegg_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Product" => {
                                    if let Some(data) = product_kegg_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Genes" => {
                                    if let Some(data) = genes_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Symbol" => {
                                    if let Some(data) = symbol_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "Organism" => {
                                    if let Some(data) = orgnism_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "AA seq" => {
                                    if let Some(data) = aa_seq_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                "NT seq" => {
                                    if let Some(data) = nt_seq_row_parsing(td.html()) {
                                        otp_string.push(data);
                                    }
                                }
                                _ => continue,
                            };
                            break;
                        }
                    } else {
                        // todo!("Сделать обработку картинки с реакцией, чтобы подгружалась");
                        continue;
                    }
                }
            }
        }
        Some(otp_string)
    }
    async fn vec_bson_to_kegg_schemas(vec_bson: Vec<Bson>) -> kegg_schemas {
        let mut tmp_otp = doc! {};

        //Собираем Document из вектора
        for elem in vec_bson {
            for (key, value) in elem.as_document().unwrap() {
                tmp_otp.insert(key, value);
            }
        }

        // Десереализуем JsonValue и дальше проверяем согласно Type
        match &tmp_otp.get("Type").unwrap() {
            Bson::String(var) => match var.as_str() {
                "CDS" => kegg_schemas::CDS(bson::Bson::Document(tmp_otp)),
                "Reaction" => kegg_schemas::Reaction(bson::Bson::Document(tmp_otp)),
                "Compound" => kegg_schemas::Compound(bson::Bson::Document(tmp_otp)),
                "Enzyme" => kegg_schemas::Enzyme(bson::Bson::Document(tmp_otp)),
                _ => kegg_schemas::Error("Type is not defind".to_string()),
            },
            _ => kegg_schemas::Error("Not string type".to_string()),
        }
    }

    async fn get_kegg(url: &str) -> kegg_schemas {
        info!(target: "get_kegg", "Запросили kegg_schemas по uri");
        Parser::vec_bson_to_kegg_schemas(Parser::parse_kegg_to_string(url).await.unwrap()).await
    }
}
