use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponseHeader {
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

impl PostResponseHeader {
    pub fn to_str(&self) -> String {
        let separator = "---";
        serde_yaml::to_string(self).unwrap() + separator
    }
}

// 記事のheader
fn serde_header(md_header: &str) -> PostHeader {
    let header: PostHeader = serde_yaml::from_str(md_header).unwrap();
    header
}

// 記事のbody
fn serde_body(md_body: &str) -> String {
    md_body.to_string()
}

// 記事(header+body -> post)
pub fn serde_post(md_post: &str) -> Post {
    let split_str: Vec<&str> = md_post.split("---").collect();

    Post {
        header: serde_header(&split_str[1]),
        body: serde_body(&split_str[2]),
    }
}
