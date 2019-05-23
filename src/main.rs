//extern crate base64;
//extern crate serde;
//extern crate serde_json;

mod token;

//use std::fs::File;
//use std::fs;
//use std::io::prelude::*;

fn main() {
    let token = token::get();
    println!("token: {}", token);
    //let mut f = File::open("myway.vob").unwrap();
    //let mut buffer = Vec::new();

    //let size = f.read_to_end(&mut buffer).unwrap();
    //let b64 = base64::encode(&buffer);
    //println!("size = {}", size);
    //fs::write("test.b64", &b64).unwrap();
    //println!("b64= {}", b64);
}
