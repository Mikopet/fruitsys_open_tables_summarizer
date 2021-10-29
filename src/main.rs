extern crate dotenv;

use dotenv::dotenv;
use scraper::{Html, Selector};
use std::env;

const LOGIN_URL: &str = "https://cloud.fruitsys.hu/raktar/index.php";
const REQUEST_URL: &str = "https://cloud.fruitsys.hu/raktar/nyitott_asztalok.php";

#[derive(Debug)]
struct Consumption {
    product: String,
    count: u8,
}

fn main() -> Result<(), reqwest::Error> {
    dotenv().ok();

    let username = env::var("FRUITSYS_USERNAME").expect("Failed to read FRUITSYS_USERNAME");
    let password = env::var("FRUITSYS_PASSWORD").expect("Failed to read FRUITSYS_PASSWORD");

    let client = reqwest::blocking::Client::builder()
        .cookie_store(true)
        .build()?;

    let _login = client
        .post(LOGIN_URL)
        .form(&[
            ("user_name", username),
            ("user_password", password),
            ("login", "Bejelentkezés".into()),
        ])
        .send()?;

    let html = client.get(REQUEST_URL).send()?.text()?;

    let document = Html::parse_document(&html);
    let table = document
        .select(&Selector::parse("table").unwrap())
        .next()
        .unwrap();

    let mut records: Vec<Consumption> = Vec::new();
    for element in table.select(&Selector::parse("tr").unwrap()) {
        let record = element.text().collect::<Vec<_>>();
        if record.len() == 3 {
            records.push(Consumption {
                product: record[0].into(),
                count: record[1].parse().unwrap(),
            });
        }
    }

    println!("{:?}", records);

    Ok(())
}
