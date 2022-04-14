use reqwest::Error;
use std::env;
use std::fs;
mod config;
mod post;

#[tokio::main]
async fn main() -> Result<(), Error> {
    config::set_default();

    let md_post = fs::read_to_string("articles/test.md").unwrap();

    let post = post::serde_post(&md_post);
    let json_post = post.jsonify();

    let client = reqwest::Client::new();
    let endpoint = env::var("QIITA_API_ENDPOINT").unwrap() + "/items";
    let authorization = String::from("Bearer ") + &env::var("QIITA_API_TOKEN").unwrap().to_string();

    println!("{:?}", &json_post);

    let res = client
        .post(endpoint)
        .header("Authorization", authorization)
        .json(&json_post)
        .send()
        .await?;

    println!("{:?}", res);

    Ok(())
}
