extern crate base64;

use crate::api;
use std::io::prelude::*;
//use std::fs;
use std::fs::File;
use crate::errors::*;

const MAX_DOC_LENGTH: usize = 1000000;

pub fn upload<'a>(token: String, files: &'a Vec<&str>) -> Result<()> {
    for file in files.iter() {
        let mut f = File::open(file).unwrap();
        let mut buffer = Vec::new();
        let mut i: usize = 0;

        let size = f.read_to_end(&mut buffer).unwrap();
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
            api::create_document(&token, "test".to_owned(), slice.to_owned())?;
            println!("slice {}", i);
        }
        println!("size = {}", size);
        //fs::write("test.b64", &b64).unwrap();
        //println!("b64= {}", b64);

        println!("upload {}", file);
    }
    Ok(())
}
