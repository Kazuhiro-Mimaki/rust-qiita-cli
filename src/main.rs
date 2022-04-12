use dotenv::dotenv;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct Post {
    title: String,
    body: String,
    private: bool,
    tags: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PostTag {
    name: String,
    versions: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    set_config();
    let post_body = read_post();
    let client = reqwest::Client::new();
    let post = set_post_body(&post_body);
    let endpoint = env::var("QIITA_API_ENDPOINT").unwrap() + "/items";
    let authorization = String::from("Bearer ") + &env::var("QIITA_API_TOKEN").unwrap().to_string();
    println!("{:?}", &post);
    let res = client
        .post(endpoint)
        .header("Authorization", authorization)
        .json(&post)
        .send()
        .await;
    // let res = reqwest::get(endpoint).await?.text().await?;
    println!("{:?}", res);
    Ok(())
}

fn set_post_body(post_body: &str) -> Post {
    let mut tag = json!({});
    tag["name"] = json!("hoge");
    tag["versions"] = json!([]);
    let tags = json!([tag]);
    let post = Post {
        title: "test".to_string(),
        body: post_body.to_string(),
        private: true,
        tags: tags.to_string(),
    };
    post
}

fn read_post() -> String {
    let contents = fs::read_to_string("articles/test.md").unwrap();
    contents
}

fn set_config() {
    dotenv().ok();
}
