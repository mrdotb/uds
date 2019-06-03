// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

// extern crate
extern crate url;

// mod
mod token;

// use
use std::process;
use clap::{App as ClapApp, Arg, SubCommand};

mod errors {
    error_chain!{
        foreign_links {
            Clap(::clap::Error);
            Io(::std::io::Error);
            Json(::serde_json::error::Error);
            Url(::url::ParseError);
            Var(::std::env::VarError);
        }
    }

    pub fn handle_error(error: &Error) {
        match error {
            //Error(ErrorKind::Io(ref io_error), _)
            //    if io_error.kind() == super::io::ErrorKind::BrokenPipe =>
            //{
            //    super::process::exit(0);
            //}
            _ => {
                use ansi_term::Colour::Red;
                eprintln!("{}: {}", Red.paint("[uds error]"), error);
            }
        };
    }
}

use crate::errors::*;
//extern crate base64;
//extern crate serde;
//extern crate serde_json;

//mod token;
//mod api;

//use std::fs::File;
//use std::fs;
//use std::str;
//use std::io::prelude::*;

fn run() -> Result<bool> {
    let matches = ClapApp::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about("Unlimited Drive Storage")
        .subcommand(SubCommand::with_name("list")
                    .about("List uploaded files.")
                   )
        .subcommand(SubCommand::with_name("download")
                    .about("Download.")
                    .arg(Arg::with_name("file")
                         .multiple(true)
                         .empty_values(false)
                        ))
        .subcommand(SubCommand::with_name("upload")
                    .about("Upload.")
                    .arg(Arg::with_name("file")
                         .multiple(true)
                         .empty_values(false)
                        ))
        .get_matches();

    match matches.subcommand() {
        ("upload", Some(matches)) =>{
            let files = matches
                .values_of("file")
                .chain_err(|| "No file argument found")?
                .collect::<Vec<_>>();

            println!("file {:?}", files);
        },
        ("download", Some(matches)) =>{
            let files = matches
                .values_of("file")
                .chain_err(|| "No file argument found")?
                .collect::<Vec<_>>();

            println!("file {:?}", files);
        },
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }
    Ok(true)
}

fn main() {
    token::get();
    let result = run();

    match result {
        Err(error) => {
            handle_error(&error);
            process::exit(1);
        }
        Ok(false) => {
            process::exit(1);
        }
        Ok(true) => {
            process::exit(0);
        }
    }
}

//api::create_folder(token.clone(), "test".to_owned());
//let mut f = File::open("img.b64").unwrap();
//let mut buffer = Vec::new();
//f.read_to_end(&mut buffer).unwrap();
//let c = str::from_utf8(&buffer).unwrap();

//let c = "test";
//api::create_document(token.access_token().secret().clone(), "test".to_owned(), c.to_owned());
//let mut f = File::open("myway.vob").unwrap();
//let mut buffer = Vec::new();

//let size = f.read_to_end(&mut buffer).unwrap();
//let b64 = base64::encode(&buffer);
//println!("size = {}", size);
//fs::write("test.b64", &b64).unwrap();
//println!("b64= {}", b64);
