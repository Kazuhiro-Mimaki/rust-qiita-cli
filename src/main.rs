use reqwest::Error;
use std::env;
use std::fs;

mod api;
mod config;
mod file;
mod post;

#[tokio::main]
async fn main() -> Result<(), Error> {
    config::set_default();
    const POST_DIR: &str = "posts";
    let post_dir_paths = fs::read_dir(POST_DIR).unwrap();

    for post_file in post_dir_paths {
        let file_path = post_file.unwrap().path();

        let md_post = file::read_file(&file_path);
        let post = post::parse_markdown(&md_post);

        let api_client = api::ApiClient {
            client: reqwest::Client::new(),
            endpoint: env::var("QIITA_API_ENDPOINT").unwrap() + "/items",
            authorization: String::from("Bearer ")
                + &env::var("QIITA_API_TOKEN").unwrap().to_string(),
        };

        let json_post = post.jsonify();
        let response: post::PostResponse;

        match post.header.id {
            // idがある場合はupdate
            Some(id) => {
                println!("Update post with id={:?}", id);
                response = api_client.patch(&id, &json_post).await;
            }
            // idがない場合はpost
            None => {
                println!("New post");
                response = api_client.post(&json_post).await;
            }
        }

        // ファイル書き込み
        let markdown_post = post::parse_http_response(&response);
        file::update(&file_path, &markdown_post.as_bytes());
    }

    Ok(())
}
