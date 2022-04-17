use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    id: String,
    title: String,
    tags: Vec<PostTag>,
    private: bool,
    updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostTag {
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub header: PostHeader,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostHeader {
    pub id: Option<String>,
    title: String,
    private: bool,
    tags: Vec<PostTag>,
    updated_at: Option<String>,
}

const SEPARATOR: &str = "---";

impl Post {
    // api post用にjson化
    pub fn jsonify(&self) -> Value {
        json!({
            "title": self.header.title,
            "private": self.header.private,
            "tags": self.header.tags,
            "body": self.body,
        })
    }
}

impl PostResponse {
    pub fn to_str(&self) -> String {
        serde_yaml::to_string(self).unwrap() + SEPARATOR
    }
}

pub fn parse_markdown(md_post: &str) -> Post {
    let split_str: Vec<&str> = md_post.split(SEPARATOR).collect();
    let header: PostHeader = serde_yaml::from_str(split_str[1]).unwrap();
    let body = String::from(split_str[2]);

    Post {
        header: header,
        body: body,
    }
}
