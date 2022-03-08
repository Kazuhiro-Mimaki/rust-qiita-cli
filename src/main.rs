use dotenv::dotenv;
use reqwest::Error;
use std::env;
// use std::fs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    // let contents = fs::read_to_string("articles/test.md").unwrap();
    let endpoint = env::var("QIITA_API_ENDPOINT").unwrap() + "/items";
    let res = reqwest::get(endpoint).await?.text().await?;
    // let items = res.text().await?;
    println!("{:?}", res);
    Ok(())
}
