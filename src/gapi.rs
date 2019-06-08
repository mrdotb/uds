#![allow(dead_code)]
extern crate reqwest;

use reqwest::multipart;

use crate::errors::*;
use crate::token;

#[derive(Deserialize, Debug)]
pub struct CreateResponse {
    id: String,
}

#[derive(Deserialize, Debug)]
pub struct ListResponse {
    files: Vec<File>
}

#[derive(Deserialize, Debug)]
struct File {
    id: String,
    #[serde(rename = "mimeType")]
    mime_type: Option<String>,
    name: Option<String>,
    properties: Option<Properties>
}

#[derive(Deserialize, Debug)]
struct Properties {
    size: u32,
    size_numeric: u32,
    encoded_size: u32,
    md5: String,
}

pub struct DriveApi {
    token: String,
    client: reqwest::Client,
}

impl DriveApi {
    pub fn new() -> Result<DriveApi> {
        let token = token::get()?;
        let client = reqwest::Client::builder()
            //.timeout(Duration::from_secs(100))
            .build()?;
        let drive_api = DriveApi{token, client};
        //drive_api.create_or_find_root_folder();

        Ok(drive_api)
    }

    fn create_or_find_root_folder(&self) -> Result<String> {
        let res = self.find_root_folder()?;

        if res.files.len() == 0 {
            let res = self.create_root_folder()?;
            Ok(res.id)
        } else {
           Ok(res.files[0].id.to_owned())
        }
    }

    pub fn create_root_folder(&self) -> Result<CreateResponse> {
        let body = json!({
            "name": "UDS root",
            "mimeType": "application/vnd.google-apps.folder",
            "properties": {
                "udsRoot": true
            }
        });

        self.create_folder(body)
    }

    fn create_folder(&self, body: serde_json::Value) -> Result<CreateResponse> {
        let url = format!(
            "https://content.googleapis.com/drive/v3/files?access_token={token}",
            token = self.token
        );

        let mut response = self.client
            .post(&url)
            .json(&body)
            .send()?;

        let response: CreateResponse = response.json()?;
        Ok(response)
    }

    pub fn create_media_folder(&self) -> Result<CreateResponse> {
        let body = json!({
            "name": "name",
            "mimeType": "application/vnd.google-apps.folder",
            "properties": {
                "uds": true,
                "size": "size",
                "size_numeric": "size_numeric",
                "encoded_size": "encoded_size",
                "md5": "md5",

            },
            //"parents": [],
        });

        self.create_folder(body)
    }

    pub fn create_document(&self) -> Result<CreateResponse> {
        let url = format!(
            "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart&access_token={token}",
            token = self.token
        );

        let body = json!({
            "mimeType": "application/vnd.google-apps.document",
            "name": "name",
            "properties": {
                "part": "content"
            },
            //"parents": [folder_id],
        });

        let body = serde_json::to_string(&body)?;

        let meta_part = multipart::Part::text(body)
            .mime_str("application/json; charset=UTF-8")?;

        let data_part = multipart::Part::text("content")
            .mime_str("text/plain").unwrap();

        let form = multipart::Form::new()
            .part("", meta_part)
            .part("", data_part);

        //println!("form {:#?}", form);
        let mut response = self.client
            .post(&url)
            .multipart(form)
            .send()?;

       //println!("response {:#?}", response);
        let response: CreateResponse = response.json()?;
        //println!("{}", response.id);
        Ok(response)
    }

    //&pageSize=1000&fields='nextPageToken, files(id, name, properties, mimeType')
    pub fn list_files(&self, query: &str, fields: &str) -> Result<ListResponse> {
        let url = format!(
            "https://content.googleapis.com/drive/v3/files?access_token={token}&q={query}&fields={fields}",
            token = self.token,
            query = query,
            fields = fields,
        );

        let response: ListResponse = self.client
            .get(&url)
            .send()?
            .json()?;

        Ok(response)
    }

    pub fn find_root_folder(&self) -> Result<ListResponse> {
        let query = "properties has {key='udsRoot' and value='true'} and trashed=false";
        let fields = "files(id)";
        let res: ListResponse = self.list_files(query, fields)?;
        Ok(res)
    }

    pub fn delete_file() {}
}
