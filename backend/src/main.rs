#![allow(warnings)]
mod parsing;
use parsing::Parser;

use regex;
use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use tokio;

#[tokio::main]
async fn main() {
    let result = Parser::get_json("https://www.genome.jp/entry/R00085").await;

    println!("{}", result);
}
