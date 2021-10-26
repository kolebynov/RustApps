use std::{str::FromStr, time::Duration};
use log::info;
use reqwest::{Body, Url, header::{AUTHORIZATION, HeaderMap}};
use serde::{Deserialize, Serialize};

pub struct IndexingApiClient {
    uri: Url,
    client: reqwest::Client, 
}

impl IndexingApiClient {
    pub fn new(mut uri: String, token: String) -> IndexingApiClient {
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .default_headers(headers)
            .build().unwrap();

        if !uri.ends_with('/') {
            uri.push('/');
        }
        
        IndexingApiClient { uri: Url::from_str(&uri).unwrap(), client }
    }

    pub async fn upload_content(&self, content: Body) -> Result<UploadContentResponse, Box<dyn std::error::Error>> {
        Ok(self.client
            .post(self.uri.join("api/v4/content/upload")?)
            .body(content)
            .send().await?
            .json::<UploadContentResponse>().await?)
    }

    pub async fn add_documents(&self, request: AddDocumentsRequest) -> Result<(), Box<dyn std::error::Error>> {
        self.client
            .post(self.uri.join("api/v4/index/document")?)
            .json(&request)
            .send().await?
            .text().await?;

        Ok(())
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UploadContentResponse {
    content_token: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDocumentsRequest {
    pub connector_tag: String,
    pub connector_version: String,
    pub items: Vec<AddDocumentRequest>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddDocumentRequest {
    pub kb_id: i32,
    pub subset_id: String,
    pub uri: String,
    pub open_url: String,
    pub language: i32,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DocumentAttributes {
    title: String,
    modify_date_time: String,
}