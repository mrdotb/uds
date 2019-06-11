#![allow(dead_code)]
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
extern crate md5;
extern crate reqwest;
extern crate url;

// mod
mod token;
mod file;
mod gapi;

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
            Reqwest(::reqwest::Error);
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

    let client = gapi::DriveApi::new()?;

    match matches.subcommand() {
        ("upload", Some(matches)) =>{
            let files = matches
                .values_of("file")
                .chain_err(|| "No file argument found")?
                .collect::<Vec<_>>();

            file::upload(client, &files)?;

            println!("file {:?}", files);
        },
        ("download", Some(matches)) =>{
            let ids = matches
                .values_of("file")
                .chain_err(|| "No file argument found")?
                .collect::<Vec<_>>();

            file::download(client, &ids)?;
            println!("ids {:?}", ids);
        },
        ("list", _) =>{
            file::list(client).unwrap();
        },
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!(),
    }
    Ok(true)
}

fn main() {
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
