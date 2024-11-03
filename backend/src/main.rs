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

    let resp = reqwest::get("https://www.kegg.jp/entry/C00383")
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
    let dev_cell_selector = Selector::parse("div.cell").unwrap();

    let main_table = fragment.select(&main_table_selector).next().unwrap();

    for tr in main_table.select(&tr_table_selector) {
        /* Здесь нужно устроить поиск внутри tr с распределением по th и td */

        /* Ниже рабочий поиск по th */

        for th in tr.select(&th_selector) {
            /* Здесь нужно вычленить span для определения названия строчки*/
            for span in th.select(&span_selector) {
                let name_of_row = span.text().collect::<Vec<&str>>()[0];

                /* Парсим сиблинг в зависимости от имени в <th>  */
                for td in tr.select(&td_selector) {
                    // println!("=====\n{}\n==========", td.html());

                    match name_of_row {
                        "Entry" => println!("{}", entry_row_parsing(td.html()).unwrap()),
                        "Name" => println!("{}", name_row_parsing(td.html()).unwrap()),
                        "Formula" => println!("{}", formula_row_parsing(td.html()).unwrap()),
                        _ => continue,
                    };
                    break;
                }
            }
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

    // println!("{:#?}", fragment.select(&span_sel).next().unwrap().text());

    let mut word_list = fragment
        .select(&span_sel)
        .next()
        .unwrap()
        .text()
        .map(|word| word.trim().to_string())
        .collect::<Vec<String>>();

    word_list = word_list[0]
        .split_whitespace()
        .map(|word| word.to_string())
        .collect();

    let tmp_otp = otp_struct {
        Entry: word_list[0].clone(),
        Type: word_list[1].clone(),
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

    // name_list.retain(|elem| elem=="");

    let tmp_otp = otp_struct { Name: name_list };

    // println!("{}",serde_json::to_string(&tmp_otp).unwrap());

    Some(serde_json::to_string(&tmp_otp).unwrap())

    // println!("{:?}", name_list);
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

    // println!("{}",serde_json::to_string(&tmp_otp).unwrap());

    Some(serde_json::to_string(&tmp_otp).unwrap())
}
