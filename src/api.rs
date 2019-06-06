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
    files: Vec<FileEnum>
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum FileEnum {
    #[serde(skip_serializing)]
    Root{id: String},
    #[serde(skip_serializing)]
    FileB(FileB),
}

#[derive(Deserialize, Debug)]
struct FileA {
    id: String,
}

#[derive(Deserialize, Debug)]
struct FileB {
    id: String,
    #[serde(rename = "mimeType")]
    mime_type: String,
    name: String,
    //properties: PropEnum
}

//#[derive(Deserialize, Debug)]
//#[serde(untagged)]
//enum PropEnum {
//    A(PropA),
//}
//
//#[derive(Deserialize, Debug)]
//struct PropA {
//    #[serde(rename = "udsRoot")]
//    uds_root: bool
//}

pub struct DriveApi {
    token: String,
    client: reqwest::Client,
}

impl DriveApi {
    pub fn new() -> Result<DriveApi> {
        let token = token::get()?;
        let client = reqwest::Client::builder().build()?;
        let drive_api = DriveApi{token, client};
        drive_api.create_or_find_root_folder();

        Ok(drive_api)
    }

    fn create_or_find_root_folder(&self) -> Result<String> {
        println!("{:#?}", self.find_root_folder().unwrap());
        let res = self.find_root_folder()?;
        if res.files.len() == 0 {
            let res = self.create_root_folder()?;
            println!("create");
            Ok(res.id)
        } else {
            match res.files[0] {
                FileEnum::Root{id: id} => {
                    println!("{:#?}", id);
                },
                _ => unreachable!()

            }
            Ok("tamere".to_owned())
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

    pub fn create_media_folder() {
    }

    pub fn create_document() {}

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

//pub fn create_root_folder(token: &str) -> Result<CreateResponse> {
//    let body = json!({
//        "name": "UDS root",
//        "mimeType": "application/vnd.google-apps.folder",
//        "properties": {
//            "udsRoot": true
//        }
//    });
//
//    create_folder(token, body)
//}

//pub fn create_media_folder(token: &str) -> Result<CreateResponse> {
//    let body = json!({
//        "name": "UDS root",
//        "mimeType": "application/vnd.google-apps.folder",
//        "properties": {
//            "udsRoot": true
//        }
//    });
//}
//
//
//fn create_folder(token: &str, body: serde_json::Value) -> Result<CreateResponse> {
//    let url = format!(
//        "https://content.googleapis.com/drive/v3/files?access_token={token}",
//        token = token
//    );
//
//    let mut response = Client::new()
//        .post(&url)
//        .json(&body)
//        .send()?;
//
//    let response: CreateResponse = response.json()?;
//    Ok(response)
//}
//
//pub fn create_document(token: &str, folder_id: String, name: String, content: String) -> Result<CreateResponse> {
//    let body = json!({
//        "mimeType": "application/vnd.google-apps.document",
//        "name": name,
//        "properties": {
//            "part": "content"
//        },
//        "parents": [folder_id],
//    });
//
//    let body = serde_json::to_string(&body).unwrap();
//
//    let meta_part = multipart::Part::text(body)
//        .mime_str("application/json; charset=UTF-8").unwrap();
//
//    let data_part = multipart::Part::text(content)
//        .mime_str("text/plain").unwrap();
//
//    let form = multipart::Form::new()
//        .part("", meta_part)
//        .part("", data_part);
//
//    let url = format!(
//        "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart&access_token={token}",
//        token = token
//    );
//
//    //println!("form {:#?}", form);
//    let mut response = Client::builder()
//    //  .timeout(Duration::from_secs(100))
//        .build()
//        .unwrap()
//        .post(&url)
//        .multipart(form)
//        .send()?;
//
//   //println!("response {:#?}", response);
//    let response: CreateResponse = response.json()?;
//    //println!("{}", response.id);
//    Ok(response)
//}
