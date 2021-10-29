extern crate dotenv;

use dotenv::dotenv;
use std::env;

const LOGIN_URL: &str = "https://cloud.fruitsys.hu/raktar/index.php";
const REQUEST_URL: &str = "https://cloud.fruitsys.hu/raktar/nyitott_asztalok.php";

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

    let body = client.get(REQUEST_URL).send()?.text();
    println!("body = {:?}", body);

    Ok(())
}
