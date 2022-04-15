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
    id: Option<String>,
    title: String,
    private: bool,
    tags: Vec<PostTag>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YamlPostHeader {
    id: Option<String>,
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

impl PostResponseHeader {
    pub fn to_str(&self) -> String {
        let separator = "---";
        serde_yaml::to_string(self).unwrap() + separator
    }
}

// 記事のheader
fn serde_header(md_header: &str) -> PostHeader {
    let yaml_post_header: YamlPostHeader = serde_yaml::from_str(md_header).unwrap();
    let tags = make_tag_struct(&yaml_post_header.tags);
    let header = PostHeader {
        id: yaml_post_header.id,
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
fn make_tag_struct(yaml_post_header_tags: &Vec<String>) -> Vec<PostTag> {
    let mut tags: Vec<PostTag> = Vec::new();
    for tag in yaml_post_header_tags.iter() {
        tags.push(PostTag {
            name: tag.to_string(),
        })
    }
    tags
}
