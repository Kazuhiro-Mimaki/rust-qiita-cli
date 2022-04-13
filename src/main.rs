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

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
struct PostHeader {
    title: String,
    private: bool,
    tags: Vec<HashMap<&'static str, String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct YamlPostHeader {
    title: String,
    private: bool,
    tags: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    set_config();
    let post = set_post(read_header(), set_body());

    let client = reqwest::Client::new();
    let endpoint = env::var("QIITA_API_ENDPOINT").unwrap() + "/items";
    let authorization = String::from("Bearer ") + &env::var("QIITA_API_TOKEN").unwrap().to_string();

    println!("{:?}", &post);

    let res = client
        .post(endpoint)
        .header("Authorization", authorization)
        .json(&post)
        .send()
        .await?;

    println!("{:?}", res);

    Ok(())
}

fn set_post(header: PostHeader, body: String) -> Value {
    let post = Post {
        title: header.title,
        private: header.private,
        tags: header.tags,
        body: body,
    };
    json!(post)
}

fn set_body() -> String {
    let content = fs::read_to_string("articles/test.md").unwrap();
    content
}

fn set_config() {
    dotenv().ok();
}

fn read_header() -> PostHeader {
    let post = fs::read_to_string("articles/test.md").unwrap();
    let split_str: Vec<&str> = post.split("---").collect();
    let yaml_post_header: YamlPostHeader = serde_yaml::from_str(&split_str[1]).unwrap();
    let tags = make_tags_hashmap_vector(yaml_post_header.tags);
    let header = PostHeader {
        title: yaml_post_header.title,
        private: yaml_post_header.private,
        tags: tags,
    };
    header
}

// ["tag1", "tag2"] から [{"name": "tag1"}, {"name": "tag2"}] を生成
fn make_tags_hashmap_vector(yaml_post_header_tags: Vec<String>) -> Vec<HashMap<&'static str, String>> {
    let mut tags: Vec<HashMap<&'static str, String>> = Vec::new();
    for tag in yaml_post_header_tags.iter() {
        let mut new_tag = HashMap::new();
        new_tag.insert("name", String::from(tag));
        tags.push(new_tag);
    }
    tags
}