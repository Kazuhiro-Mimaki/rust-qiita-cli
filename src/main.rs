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

    let post = post::serde_post(&md_post);
    let json_post = post.jsonify();

    let client = reqwest::Client::new();
    let endpoint = env::var("QIITA_API_ENDPOINT").unwrap() + "/items";
    let authorization = String::from("Bearer ") + &env::var("QIITA_API_TOKEN").unwrap().to_string();

    println!("{:?}", &json_post);

    let post_res_header = client
        .post(endpoint)
        .header("Authorization", authorization)
        .json(&json_post)
        .send()
        .await?
        .json::<post::PostResponseHeader>()
        .await?;

    // let res = client
    //     .get(endpoint)
    //     .header("Authorization", authorization)
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    println!("{:?}", post_res_header);

    // ファイル書き込み
    let mut f = fs::File::create("articles/res.md").unwrap();
    let new_content = post_res_header.to_str() + &post.body;
    f.write_all(new_content.as_bytes()).unwrap();

    Ok(())
}
