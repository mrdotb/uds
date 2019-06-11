#![allow(dead_code)]
extern crate base64;

use crate::gapi;
use std::io::prelude::*;
use md5;
//use std::fs;
use std::fs::File;
use crate::errors::*;

const MAX_DOC_LENGTH: usize = 1000000;

pub fn upload<'a>(api: gapi::DriveApi, files: &'a Vec<&str>) -> Result<()> {
    for file in files.iter() {
        println!("upload {}", file);

        let mut f = File::open(file)?;
        let mut buffer = Vec::new();
        let mut i: usize = 0;
        let size = f.read_to_end(&mut buffer).unwrap();
        let digest = md5::compute(&buffer);
        let md5_hash = format!("{:x}", digest);
        let encoded_size = size * (4/ 3);
        let properties = gapi::Properties{
            size: size.to_string(), encoded_size: encoded_size.to_string(), md5: md5_hash
        };

        let res = api.create_media_folder(file, properties)?;
        println!("{}", res.id);

        let b64 = base64::encode(&buffer);
        while i < b64.len() {
            let j = i;
            let slice;

            i += MAX_DOC_LENGTH;
            if i > b64.len() {
                slice = &b64[j..];
            } else {
                slice = &b64[j..i];
            }
            //println!("slice {}", slice);
            api.create_document(&res.id, i.to_string(), slice.to_owned()).unwrap();
        }
        //fs::write("test.b64", &b64).unwrap();
        //println!("b64= {}", b64);
    }
    Ok(())
}

pub fn download<'a>(api: gapi::DriveApi, ids: &'a Vec<&str>) -> Result<()> {
    for id in ids.iter() {
        let res = api.find_file_chunk(id)?;
        println!("{:#?}", res.files);
    }
    Ok(())
}

pub fn list(api: gapi::DriveApi) -> Result<()> {
    let res = api.find_uploaded_files()?;
    println!("{:#?}", res);
    Ok(())
}
