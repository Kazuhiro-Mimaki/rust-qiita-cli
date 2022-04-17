use crate::post;
use serde_json::Value;

#[derive(Debug, Default)]
pub struct ApiClient {
    pub client: reqwest::Client,
    pub endpoint: String,
    pub authorization: String,
}

impl ApiClient {
    pub async fn post(&self, json_post: &Value) -> post::PostResponse {
        let response = self
            .client
            .post(&self.endpoint)
            .header("Authorization", &self.authorization)
            .json(json_post)
            .send()
            .await
            .unwrap()
            .json::<post::PostResponse>()
            .await
            .unwrap();
        response
    }

    pub async fn patch(&self, post_id: &str, json_post: &Value) -> post::PostResponse {
        let response = self
            .client
            .patch(format!("{}{}{}", &self.endpoint, "/", post_id))
            .header("Authorization", &self.authorization)
            .json(json_post)
            .send()
            .await
            .unwrap()
            .json::<post::PostResponse>()
            .await
            .unwrap();
        response
    }
}

// response = client
// .post(endpoint)
// .header("Authorization", authorization)
// .json(&json_post)
// .send()
// .await?
// .json::<post::PostResponse>()
// .await?;
