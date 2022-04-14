use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct Post {
    header: PostHeader,
    body: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(bound(deserialize = "'de: 'static"))]
pub struct PostHeader {
    title: String,
    private: bool,
    tags: Vec<HashMap<&'static str, String>>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct YamlPostHeader {
    title: String,
    private: bool,
    tags: Vec<String>,
}

impl Post {
    // api post用にjson化
    pub fn jsonify(&self) -> Value {
        json!({
            "title": self.header.title,
            "private": self.header.private,
            "tags": self.header.tags,
            "body": self.body
        })
    }
}

// 記事のheader
fn serde_header(md_header: &str) -> PostHeader {
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
