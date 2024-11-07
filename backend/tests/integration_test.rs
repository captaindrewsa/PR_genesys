/* #[test]
fn all_fact_check() {
    let url_kegg = vec![
        "https://www.genome.jp/entry/2.7.1.40",
        "https://www.genome.jp/entry/R00200",
        "https://www.genome.jp/entry/C00002",
        "https://www.genome.jp/entry/hsa:5315",
    ];

    let mut dabas = Kegg_database {
        database: mongodb::Client::with_uri_str("mongodb://127.0.0.1:27017")
            .await
            .unwrap()
            .database("kegobb"),
    };

    let mut result: bool = false;

    for elem in url_kegg {
        let tmp_doc = Parser::get_kegg(elem).await;
        match dabas.add_kegg(tmp_doc).await {
            Ok(_) => result = true,
            Err(_) => result = false,
        };
    }

    assert!(result);
} */
