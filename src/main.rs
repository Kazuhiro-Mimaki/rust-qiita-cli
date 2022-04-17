use reqwest::Error;
use std::env;
use std::fs;
use std::io::Write;

mod config;
mod post;

#[tokio::main]
async fn main() -> Result<(), Error> {
    config::set_default();

    let md_post = fs::read_to_string("articles/res.md").unwrap();
    let post = post::parse_markdown(&md_post);
    let json_post = post.jsonify();

    let client = reqwest::Client::new();
    let endpoint = env::var("QIITA_API_ENDPOINT").unwrap() + "/items";
    let authorization = String::from("Bearer ") + &env::var("QIITA_API_TOKEN").unwrap().to_string();

    let response: post::PostResponse;

    match post.header.id {
        // idがある場合はupdate
        Some(id) => {
            println!("Update post with id={:?}", id);

            response = client
                .patch(format!("{}{}{}", endpoint, "/", &id))
                .header("Authorization", authorization)
                .json(&json_post)
                .send()
                .await?
                .json::<post::PostResponse>()
                .await?;
        }
        // idがない場合はpost
        None => {
            println!("New post");

            response = client
                .post(endpoint)
                .header("Authorization", authorization)
                .json(&json_post)
                .send()
                .await?
                .json::<post::PostResponse>()
                .await?;
        }
    }

    // ファイル書き込み
    let mut f = fs::File::create("articles/res.md").unwrap();
    let new_content = response.to_str() + &post.body;
    f.write_all(new_content.as_bytes()).unwrap();

    // let res = client
    //     .get(ENDPOINT)
    //     .header("Authorization", Authorization)
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    Ok(())
}
