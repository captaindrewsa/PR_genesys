#![allow(warnings)]

use std::collections::HashMap;

use regex;
use reqwest;
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use serde_json;
use tokio;

#[tokio::main]
async fn main() {
    /*
    th > SPAN
    td > DIV.CELL
     */

    let resp = reqwest::get("https://www.genome.jp/entry/hsa:377841")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
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
                            "Entry" => if let Some(var) = entry_row_parsing(td.html()) {
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Name" => if let Some(var) = name_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Formula" => if let Some(var) = formula_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Exact mass" => if let Some(var) = exact_mass_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Mol weight" => if let Some(var) = mol_weight_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            // "Structure" => todo!("Сделать загрузку Mol File"),
                            "Reaction" => if let Some(var) = reaction_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Enzyme" => if let Some(var) = enzyme_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Pathway" => if let Some(var) = pathway_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Module" => if let Some(var) = module_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Definition" => if let Some(var) = definition_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Equation" => if let Some(var) = equation_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Reaction(IUBMB)" => if let Some(var) = reaction_iubmb_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Reaction(KEGG)" => if let Some(var) = reaction_kegg_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Substrate" => if let Some(var) = substrate_kegg_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Product" => if let Some(var) = product_kegg_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            "Genes" => if let Some(var) = genes_row_parsing(td.html()){
                                println!("{}", var);
                                
                            } else {
                                println!("None");
                            },
                            _ => continue,
                        };
                        break;
                    }
                } else {
                    // todo!("Сделать обработку картинки с реакцией, чтобы подгружалась");

                    continue;
                }
            }

            /* Парсим сиблинг в зависимости от имени в <th>  */
            // println!("=================");
        }
    }
}

fn entry_row_parsing(html: String) -> Option<String> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Entry: String,
        Type: String,
    }

    let fragment = Html::parse_fragment(&html);
    let span_sel = Selector::parse("table.w1 td.tal span").unwrap();

    let reg_entry = regex::Regex::new(r"[A-Z]{0,2}\s??[0-9\.]{5,16}").unwrap();
    let reg_type = regex::Regex::new(r"[A-Za-z]{3,15}").unwrap();

    let mut word_list = fragment
        .select(&span_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.trim().to_string())
        .collect::<Vec<String>>();

    let word_list = vec![
        reg_entry.find(&word_list[0]).unwrap().as_str(),
        reg_type.find(&word_list[0]).unwrap().as_str(),
    ];

    let tmp_otp = otp_struct {
        Entry: word_list[0].clone().to_string(),
        Type: word_list[1].clone().to_string(),
    };

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn name_row_parsing(html: String) -> Option<String> {
    /* Парсим блок напротив Name в Compound */

    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Name: Vec<String>,
    };

    let fragment = Html::parse_fragment(&html);
    let div_cell_sel = Selector::parse("div.cel").unwrap();

    let name_list = fragment
        .select(&div_cell_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.to_string())
        .collect::<Vec<String>>()
        .join("")
        .trim()
        .split(";")
        .map(|word| word.trim().to_string())
        .collect::<Vec<String>>();

    let tmp_otp = otp_struct { Name: name_list };

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn formula_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn exact_mass_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn mol_weight_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn reaction_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn enzyme_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn pathway_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn module_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn definition_row_parsing(html: String) -> Option<String> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Substrate: Vec<String>,
        Product: Vec<String>,
        Reversible: bool,
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

    let reversible = definition_string.contains("<=>");

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

    let tmp_otp = otp_struct {
        Substrate: substrate,
        Product: products,
        Reversible: reversible,
    };

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn equation_row_parsing(html: String) -> Option<String> {
    #[derive(Serialize, Deserialize, Debug)]
    struct otp_struct {
        Substrate: Vec<String>,
        Product: Vec<String>,
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

    let tmp_otp = otp_struct {
        Substrate: reagents[0].clone(),
        Product: reagents[1].clone(),
    };

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn reaction_iubmb_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn reaction_kegg_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn substrate_kegg_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn product_kegg_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
fn genes_row_parsing(html: String) -> Option<String> {
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

    Some(serde_json::to_string(&tmp_otp).unwrap())
}

