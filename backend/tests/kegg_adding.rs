use tokio;
/*
#[tokio::test]
pub async fn all_things() {
    loger();

    let url_kegg = vec![
        // "https://www.genome.jp/entry/7.2.2.13",
        // "https://www.genome.jp/entry/1.1.3.8",
        // "https://www.genome.jp/entry/4.1.3.3",
        // "https://www.genome.jp/entry/4.1.3.38",
        // "https://www.genome.jp/entry/7.5.2.3",
        "https://www.genome.jp/entry/7.6.2.1",
        "https://www.kegg.jp/entry/C07277",
        // "https://www.kegg.jp/entry/C11907",
        // "https://www.kegg.jp/entry/C00005",
        // "https://www.kegg.jp/entry/C00080",
    ];

    let mut dabas = Kegg_database {
        database: mongodb::Client::with_uri_str("mongodb://127.0.0.1:27017")
            .await
            .unwrap()
            .database("kegobb"),
    };

    for elem in url_kegg {
        let tmp_doc = Parser::get_kegg(elem).await;
        match dabas.add_kegg(tmp_doc).await {
            Ok(_) => {
                assert!(true);
                continue;
            }
            Err(_) => {
                assert!(false);
                continue;
            }
        };
    }
}
 */
