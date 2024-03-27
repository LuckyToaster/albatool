use reqwest::Error;
use tokio;
use scraper::{
    Html, 
    Selector
};

const GENOME_JP: &str = "https://www.genome.jp/dbget-bin/www_bfind_sub?mode=bfind&max_hit=1000&locale=en&serv=kegg&dbkey=genes&keywords=";
const PAGE: &str = "&page=";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();
    let div = Selector::parse("div").unwrap();
    let a = Selector::parse("a").unwrap();
    let mut results = Vec::<String>::new();

    let url = String::from(GENOME_JP) + args[1].as_str() + PAGE + "1"; // page_counter.to_string().as_str();
    let html = reqwest::get(url).await?.text().await?;
    let doc = Html::parse_document(html.clone().as_str());

    for div in doc.select(&div) {
        for a in div.select(&a) {
            results.push(format!(
                "https://www.genome.jp{}",
                a.value().attr("href").unwrap().to_string()
            ));
        }
    }

    for result in results { println!("{}", result); }

    Ok(())
}
