use dotenv::dotenv;
use reqwest::Error;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
struct Post {
    title: String,
    body: String,
    private: bool,
    tags: Vec<HashMap<&'static str, String>>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    set_config();
    let post = set_post();

    let client = reqwest::Client::new();
    let endpoint = env::var("QIITA_API_ENDPOINT").unwrap() + "/items";
    let authorization = String::from("Bearer ") + &env::var("QIITA_API_TOKEN").unwrap().to_string();

    let res = client
        .post(endpoint)
        .header("Authorization", authorization)
        .json(&post)
        .send()
        .await?;

    println!("{:?}", res);

    Ok(())
}

fn set_post() -> Value {
    let post = Post {
        title: set_title(),
        body: set_body(),
        private: set_is_private(),
        tags: set_tags(),
    };
    json!(post)
}

fn set_title() -> String {
    String::from("test")
}

fn set_body() -> String {
    let content = fs::read_to_string("articles/test.md").unwrap();
    content
}

fn set_is_private() -> bool {
    true
}

fn set_tags() -> Vec<HashMap<&'static str, String>> {
    let mut tag = HashMap::new();
    tag.insert("name", String::from("test"));
    // 任意の数tagをリストに詰める
    let mut tags = Vec::new();
    tags.push(tag);
    tags
}

fn set_config() {
    dotenv().ok();
}
