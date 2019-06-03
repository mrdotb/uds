extern crate reqwest;

use reqwest::Client;
use reqwest::multipart;

use crate::errors::*;

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    id: String,
    name: String,
}

pub fn create_folder(token: String, name: String) -> Result<ApiResponse> {
    let body = json!({
        "mimeType": "application/vnd.google-apps.folder",
        "name": name
    });
    let url = format!(
        "https://content.googleapis.com/drive/v3/files?access_token={token}",
        token = token
    );

    let mut response = Client::new()
        .post(&url)
        .json(&body)
        .send()?;

    let response: ApiResponse = response.json()?;
    //println!("{}", response.id);
    Ok(response)
}

pub fn create_document(token: &str, name: String, content: String) -> Result<ApiResponse> {
    let body = json!({
        "mimeType": "application/vnd.google-apps.document",
        "name": name,
        "properties": {
            "part": "content"
        },
    });

    let body = serde_json::to_string(&body).unwrap();

    let meta_part = multipart::Part::text(body)
        .mime_str("application/json; charset=UTF-8").unwrap();

    let data_part = multipart::Part::text(content)
        .mime_str("text/plain").unwrap();

    let form = multipart::Form::new()
        .part("", meta_part)
        .part("", data_part);

    let url = format!(
        "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart&access_token={token}",
        token = token
    );

    //println!("form {:#?}", form);
    let mut response = Client::builder()
    //  .timeout(Duration::from_secs(100))
        .build()
        .unwrap()
        .post(&url)
        .multipart(form)
        .send()?;

   //println!("response {:#?}", response);
    let response: ApiResponse = response.json()?;
    //println!("{}", response.id);
    Ok(response)
}
