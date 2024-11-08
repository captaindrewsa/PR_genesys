use bson::Bson;
use log::{info, trace, warn};
use regex;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json;
use std::{collections::HashMap, vec};

use crate::parsing::schemas;

pub fn entry_row_parsing(html: String) -> Option<Bson> {
    trace!("Инициировали парсинг поля Entry");
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Entry: String,
        Type: String,
    }

    

    let fragment = Html::parse_fragment(&html);
    let span_sel = Selector::parse("table.w1 td.tal span").unwrap();

    let reg_entry = regex::Regex::new(r"[A-Z]{0,2}\s??[0-9\.]{4,16}").unwrap();
    let reg_type = regex::Regex::new(r"[A-Za-z]{3,8}").unwrap();

    
    let mut word_list = {
        if let Some(elem) = fragment.select(&span_sel).next(){
            trace!("Scraper распознал поле Entry");
            elem.text()
        .map(|word| word.trim().to_string())
        .collect::<Vec<String>>()
    } else {
        warn!("Scraper не распознал поле Entry");
        vec![]
    }};
    

    let word_list = {
        trace!("Распознавание Entry и Type в списке");
        vec![
        reg_entry.find(&word_list[0]).unwrap().as_str(),
        reg_type.find(&word_list[0]).unwrap().as_str(),
    ]};

    let tmp_otp = otp_struct {
        Entry: word_list[0].clone().to_string(),
        Type: word_list[1].clone().to_string(),
    };

    trace!("Поле Entry обработано. Возврат Some(BSON)");
    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn name_row_parsing(html: String) -> Option<Bson> {
    /* Парсим блок напротив Name в Compound */
    trace!("Инициировали парсинг поля Name");

    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Name: Vec<String>,
    };

    let fragment = Html::parse_fragment(&html);
    let div_cell_sel = Selector::parse("div.cel").unwrap();


    let name_list = {
        if let Some(elem) = fragment.select(&div_cell_sel).next(){
            trace!("Scraper распознал поле ");
            elem
            .text()
            .map(|word| word.to_string())
            .collect::<Vec<String>>()
            .join("")
            .trim()
            .split(";")
            .map(|word| word.trim().to_string())
            .collect::<Vec<String>>()


        } else {
            warn!("Scraper не распознал поле Entry");
            vec![]
        }};

    let tmp_otp = otp_struct { Name: name_list };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn formula_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Formula: String,
    };

    let fragment = Html::parse_fragment(&html);
    let div_cell_sel = Selector::parse("div.cel").unwrap();

    let formula = fragment
        .select(&div_cell_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.to_string())
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .to_string();

    let tmp_otp = otp_struct { Formula: formula };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn exact_mass_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Exact_mass: f32,
    };

    let fragment = Html::parse_fragment(&html);
    let div_cell_sel = Selector::parse("div.cel").unwrap();

    let mass: f32 = fragment
        .select(&div_cell_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.to_string())
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .parse()
        .unwrap_or(0.000);

    let tmp_otp = otp_struct { Exact_mass: mass };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn mol_weight_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Mol_weight: f32,
    };

    let fragment = Html::parse_fragment(&html);
    let div_cell_sel = Selector::parse("div.cel").unwrap();

    let weight: f32 = fragment
        .select(&div_cell_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.to_string())
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .parse()
        .unwrap_or(0.000);

    let tmp_otp = otp_struct { Mol_weight: weight };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn reaction_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Reaction: Vec<String>,
    };

    let re = regex::Regex::new(r"R[0-9]{5}").unwrap();

    let fragment = Html::parse_fragment(&html);
    let div_cell_sel = Selector::parse("div.cel").unwrap();

    let reactions_string = fragment
        .select(&div_cell_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.to_string())
        .collect::<String>();

    let reaction_list = re
        .find_iter(&reactions_string)
        .map(|reac| reac.as_str().to_string())
        .collect::<Vec<String>>();

    let tmp_otp = otp_struct {
        Reaction: reaction_list,
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn enzyme_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Enzyme: Vec<String>,
    }

    let re = regex::Regex::new(r"[0-9\-]{1,4}\.[0-9\-]{1,4}\.[0-9\-]{1,4}\.[0-9\-]{1,4}").unwrap();

    let fragment = Html::parse_fragment(&html);
    let div_cell_sel = Selector::parse("div.cel").unwrap();

    let enzymes_string = fragment
        .select(&div_cell_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.to_string())
        .collect::<String>();

    let enzymes_list = re
        .find_iter(&enzymes_string)
        .map(|reac| reac.as_str().to_string())
        .collect::<Vec<String>>();

    let tmp_otp = otp_struct {
        Enzyme: enzymes_list,
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn pathway_row_parsing(html: String) -> Option<Bson> {
    /* Бегаем по table.w1, берем там span и td в один вектор, которые потом стакаем */
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Pathway: Vec<Vec<String>>,
    };

    let fragment = Html::parse_fragment(&html);
    let table_sel = Selector::parse("table.w1").unwrap();
    let body_sel = Selector::parse("body").unwrap();

    let mut final_vec_of_map: Vec<Vec<String>> = Vec::new();

    for table in fragment.select(&table_sel) {
        //Здесь бежим по строчкам-табличкам

        let fragment = Html::parse_document(&table.inner_html().to_string()); //Обернули таблички в новый парсер

        let tmp_vec = fragment
            .select(&body_sel)
            .next()
            .unwrap()
            .text()
            .map(|word| word.to_string())
            .collect::<Vec<String>>()
            .join("")
            .split("\u{a0}\u{a0}")
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        final_vec_of_map.push(tmp_vec);
    }

    let tmp_otp = otp_struct {
        Pathway: final_vec_of_map,
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn module_row_parsing(html: String) -> Option<Bson> {
    /* Бегаем по table.w1, берем там span и td в один вектор, которые потом стакаем */
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Module: Vec<Vec<String>>,
    };

    let fragment = Html::parse_fragment(&html);
    let table_sel = Selector::parse("table.w1").unwrap();
    let body_sel = Selector::parse("body").unwrap();

    let mut final_vec_of_module: Vec<Vec<String>> = Vec::new();

    for table in fragment.select(&table_sel) {
        //Здесь бежим по строчкам-табличкам

        let fragment = Html::parse_document(&table.inner_html().to_string()); //Обернули таблички в новый парсер

        let tmp_vec = fragment
            .select(&body_sel)
            .next()
            .unwrap()
            .text()
            .map(|word| word.to_string())
            .collect::<Vec<String>>()
            .join("")
            .split("\u{a0}\u{a0}")
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        final_vec_of_module.push(tmp_vec);
    }

    let tmp_otp = otp_struct {
        Module: final_vec_of_module,
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn definition_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Definition: schemas::definition,
    }

    let fragment = Html::parse_fragment(&html);
    let div_cell_sel = Selector::parse("div.cel").unwrap();

    let definition_string = fragment
        .select(&div_cell_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.trim().to_string())
        .collect::<String>();

    let reagents = definition_string
        .split("<=>")
        .map(|var| var.to_string())
        .collect::<Vec<String>>();

    let substrate = reagents[0]
        .split(" + ")
        .map(|var| var.trim().to_string())
        .collect::<Vec<String>>();
    let products = reagents[1]
        .split(" + ")
        .map(|var| var.trim().to_string())
        .collect::<Vec<String>>();

    let tmp_otp = schemas::definition {
        Substrate: substrate,
        Product: products,
    };

    let tmp_otp = otp_struct {
        Definition: tmp_otp,
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn equation_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Equation: schemas::equation,
    }

    let fragment = Html::parse_fragment(&html);
    let div_cell_sel = Selector::parse("div.cel").unwrap();

    let reg = regex::Regex::new(r"C[0-9]{5}").unwrap();

    let equation_string = fragment
        .select(&div_cell_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.to_string())
        .collect::<String>();

    let reagents = equation_string
        .split("=")
        .map(|half_string| {
            reg.find_iter(half_string)
                .map(|half_string| half_string.as_str().to_string())
                .collect::<Vec<String>>()
        })
        .collect::<Vec<Vec<String>>>();

    let tmp_otp = schemas::equation {
        Substrate: reagents[0].clone(),
        Product: reagents[1].clone(),
    };

    let tmp_otp = otp_struct { Equation: tmp_otp };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn reaction_iubmb_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Reaction_IUBMB: Vec<String>,
    }

    let fragment = Html::parse_fragment(&html);
    let a_selec = Selector::parse(r"a").unwrap();

    let mut list_reactions: Vec<String> = Vec::new();

    for elem in fragment.select(&a_selec) {
        list_reactions.push(
            elem.text()
                .map(|word| word.trim().to_string())
                .collect::<_>(),
        );
    }

    let tmp_otp = otp_struct {
        Reaction_IUBMB: list_reactions,
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn reaction_kegg_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Reaction_KEGG: Vec<String>,
    }

    let fragment = Html::parse_fragment(&html);
    let a_selec = Selector::parse(r"a").unwrap();

    let mut list_reactions: Vec<String> = Vec::new();

    for elem in fragment.select(&a_selec) {
        list_reactions.push(
            elem.text()
                .map(|word| word.trim().to_string())
                .collect::<_>(),
        );
    }

    let tmp_otp = otp_struct {
        Reaction_KEGG: list_reactions,
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn substrate_kegg_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Substrate: Vec<String>,
    }

    let fragment = Html::parse_fragment(&html);
    let a_selec = Selector::parse(r"a").unwrap();

    let mut list_substrate: Vec<String> = Vec::new();

    for elem in fragment.select(&a_selec) {
        list_substrate.push(
            elem.text()
                .map(|word| word.trim().to_string())
                .collect::<_>(),
        );
    }

    let tmp_otp = otp_struct {
        Substrate: list_substrate,
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn product_kegg_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Product: Vec<String>,
    }

    let fragment = Html::parse_fragment(&html);
    let a_selec = Selector::parse(r"a").unwrap();

    let mut list_product: Vec<String> = Vec::new();

    for elem in fragment.select(&a_selec) {
        list_product.push(
            elem.text()
                .map(|word| word.trim().to_string())
                .collect::<_>(),
        );
    }

    let tmp_otp = otp_struct {
        Product: list_product,
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn genes_row_parsing(html: String) -> Option<Bson> {
    /* Бегаем по table.w1, берем там span и td в один вектор, которые потом стакаем */
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Genes: HashMap<String, Vec<String>>,
    }

    let fragment = Html::parse_fragment(&html);
    let table_sel = Selector::parse("table.w1").unwrap();
    let body_sel = Selector::parse("body").unwrap();

    let mut final_map_of_module: HashMap<String, Vec<String>> = HashMap::new();

    for table in fragment.select(&table_sel) {
        //Здесь бежим по строчкам-табличкам

        let fragment = Html::parse_document(&table.inner_html().to_string()); //Обернули таблички в новый парсер

        let pair_vec = fragment
            .select(&body_sel)
            .next()
            .unwrap()
            .text()
            .map(|word| word.trim().to_string())
            .collect::<Vec<String>>()
            .join("")
            .split(":")
            .map(|word| word.to_string())
            .collect::<Vec<String>>();

        final_map_of_module.insert(
            pair_vec[0].clone(),
            pair_vec[1]
                .split(" ")
                .map(|var| var.trim().to_string())
                .collect::<Vec<String>>(),
        );
    }

    let tmp_otp = otp_struct {
        Genes: final_map_of_module,
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn symbol_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Symbol: Vec<String>,
    }

    let fragment = Html::parse_fragment(&html);
    let div_sel = Selector::parse(r"div.cel").unwrap();

    let tmp_vec = fragment
        .select(&div_sel)
        .next()
        .unwrap()
        .text()
        .map(|var| var.trim().to_string())
        .collect::<String>()
        .split(",")
        .map(|var| var.trim().to_string())
        .collect::<Vec<String>>();

    let tmp_otp = otp_struct { Symbol: tmp_vec };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn orgnism_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Organism: Vec<String>,
    };

    let fragment = Html::parse_fragment(&html);
    let div_sel = Selector::parse("div.cel").unwrap();

    let tmp_vec = fragment
        .select(&div_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.trim().to_string())
        .collect::<Vec<String>>()
        .join("  ")
        .trim()
        .split("  ")
        .map(|var| var.to_string())
        .collect();

    let tmp_otp = otp_struct { Organism: tmp_vec };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn aa_seq_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        AA_seq: String,
    };

    let fragment = Html::parse_fragment(&html);
    let td_sel = Selector::parse("html").unwrap();

    let reg = regex::Regex::new(r"[A-Z]{3,}").unwrap();

    let tmp = fragment
        .select(&td_sel)
        .next()
        .unwrap()
        .text()
        .map(|var| var.trim().to_string())
        .collect::<String>();

    let tmp_otp = otp_struct {
        AA_seq: reg.find(&tmp).unwrap().as_str().to_string(),
    };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
pub fn nt_seq_row_parsing(html: String) -> Option<Bson> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        NT_seq: String,
    };

    let fragment = Html::parse_fragment(&html);
    let html_sel = Selector::parse("html").unwrap();

    let reg = regex::Regex::new(r"(a|t|g|c|u){6,}").unwrap();

    let word_list = fragment
        .select(&html_sel)
        .next()
        .unwrap()
        .text()
        .map(|var| var.trim().to_string())
        .collect::<Vec<String>>()
        .join(" ");

    let mut tmp = String::new();

    for (nt_seq, [_]) in reg.captures_iter(&word_list).map(|word| word.extract()) {
        tmp.push_str(nt_seq);
    }

    let tmp_otp = otp_struct { NT_seq: tmp };

    Some(bson::to_bson(&tmp_otp).unwrap())
}
