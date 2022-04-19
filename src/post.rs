use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;

use crate::config;

// ====================
// struct
// ====================

#[derive(Debug, Serialize, Deserialize)]
pub struct PostResponse {
    id: Option<String>,
    title: String,
    tags: Vec<PostTag>,
    private: bool,
    updated_at: Option<String>,
    body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
    tags: Vec<PostTag>,
    private: bool,
    updated_at: Option<String>,
}

// ====================
// impl
// ====================

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

impl PostHeader {
    pub fn to_md_header(&self) -> String {
        serde_yaml::to_string(self).unwrap() + config::SEPARATOR
    }
}

impl PostResponse {
    fn header(&self) -> PostHeader {
        PostHeader {
            id: self.id.clone(),
            title: self.title.clone(),
            tags: self.tags.clone(),
            private: self.private,
            updated_at: self.updated_at.clone(),
        }
    }
}

// ====================
// function
// ====================

pub fn parse_markdown(md_post: &str) -> Post {
    let split_str: Vec<&str> = md_post.split(config::SEPARATOR).collect();
    let header: PostHeader = serde_yaml::from_str(split_str[1]).unwrap();
    let body = String::from(split_str[2]);

    Post {
        header: header,
        body: body,
    }
}

pub fn parse_http_response(res: &PostResponse) -> String {
    let header = &res.header();
    let markdown_post = header.to_md_header() + &res.body;
    markdown_post
}
