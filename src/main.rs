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
    header: PostHeader,
    body: String,
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
    let json_post = make_json_post(read_post());

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

// api post用にjson化
fn make_json_post(post: Post) -> Value {
    json!({
        "title": post.header.title,
        "private": post.header.private,
        "tags": post.header.tags,
        "body": post.body
    })
}

fn set_config() {
    dotenv().ok();
}

// 記事のheader
fn read_header(md_header: &str) -> PostHeader {
    let yaml_post_header: YamlPostHeader = serde_yaml::from_str(md_header).unwrap();
    let tags = make_tags_hashmap_vector(yaml_post_header.tags);
    let header = PostHeader {
        title: yaml_post_header.title,
        private: yaml_post_header.private,
        tags: tags,
    };
    header
}

// 記事のbody
fn read_body(md_body: &str) -> String {
    md_body.to_string()
}

// 記事(header+body)
fn read_post() -> Post {
    let post = fs::read_to_string("articles/test.md").unwrap();
    let split_str: Vec<&str> = post.split("---").collect();

    let header = read_header(&split_str[1]);
    let body = read_body(&split_str[2]);
    Post {
        header: header,
        body: body,
    }
}

// ["tag1", "tag2"] から [{"name": "tag1"}, {"name": "tag2"}] を生成
fn make_tags_hashmap_vector(
    yaml_post_header_tags: Vec<String>,
) -> Vec<HashMap<&'static str, String>> {
    let mut tags: Vec<HashMap<&'static str, String>> = Vec::new();
    for tag in yaml_post_header_tags.iter() {
        let mut new_tag = HashMap::new();
        new_tag.insert("name", String::from(tag));
        tags.push(new_tag);
    }
    tags
}
