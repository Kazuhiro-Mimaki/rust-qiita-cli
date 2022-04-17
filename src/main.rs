use reqwest::Error;
use std::env;
use std::fs;
use std::io::Write;

mod config;
mod post;
mod api;

#[tokio::main]
async fn main() -> Result<(), Error> {
    config::set_default();

    let md_post = fs::read_to_string("articles/res.md").unwrap();
    let post = post::parse_markdown(&md_post);
    let json_post = post.jsonify();

    let api_client = api::ApiClient {
        client: reqwest::Client::new(),
        endpoint: env::var("QIITA_API_ENDPOINT").unwrap() + "/items",
        authorization: String::from("Bearer ") + &env::var("QIITA_API_TOKEN").unwrap().to_string(),
    };

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
    let mut f = fs::File::create("articles/res.md").unwrap();
    f.write_all(markdown_post.as_bytes()).unwrap();

    // let res = client
    //     .get(ENDPOINT)
    //     .header("Authorization", Authorization)
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    Ok(())
}
