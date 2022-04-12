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
    let post = set_post();
    println!("{:?}", &post);

    let client = reqwest::Client::new();
    let endpoint = env::var("QIITA_API_ENDPOINT").unwrap() + "/items";
    let authorization = String::from("Bearer ") + &env::var("QIITA_API_TOKEN").unwrap().to_string();

    let res = client
        .post(endpoint)
        .header("Authorization", authorization)
        .json(&post)
        .send()
        .await;
    println!("{:?}", res);

    Ok(())
}

fn set_post() -> Post {
    let post = Post {
        title: set_title(),
        body: set_body(),
        private: set_is_private(),
        tags: set_tags(),
    };
    post
}

fn set_title() -> String {
    String::from("test")
}

fn set_body() -> String {
    let contents = fs::read_to_string("articles/test.md").unwrap();
    contents
}

fn set_is_private() -> bool {
    true
}

fn set_tags() -> String {
    let mut tag = json!({});
    tag["name"] = json!("test");
    tag["versions"] = json!([]);
    // 任意の数tagをリストに詰める
    let tags = json!([tag, tag, tag]);
    tags.to_string()
}

fn set_config() {
    dotenv().ok();
}
