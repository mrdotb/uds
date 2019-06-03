extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate url;
extern crate serde_json;

use crate::errors::*;
use serde_json::{Value};
use oauth2::basic::BasicClient;
use oauth2::basic::BasicTokenResponse;
use oauth2::*;
use oauth2::prelude::*;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl, 
};
use std::env;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::net::TcpListener;
use url::{Url};

fn get_secrets() -> Result<(String, String)> {
    match File::open("credentials.json") {
        Ok(mut file) => {
            let mut serialized_json = String::new();

            file.read_to_string(&mut serialized_json)?;
            let credentials: Value = serde_json::from_str(&serialized_json)?;
            let client_id: String = credentials["installed"]["client_id"]
                .as_str().unwrap().to_owned();
            let client_secret: String = credentials["installed"]["client_secret"]
                .as_str().unwrap().to_owned();
            Ok((client_id, client_secret))
        },
        Err(_err) => {
            let client_id = env::var("GOOGLE_CLIENT_ID")?;
            let client_secret = env::var("GOOGLE_CLIENT_SECRET")?;
            Ok((client_id, client_secret))
        }
    }
}

fn create_client() -> Result<BasicClient> {
    let (client_id, client_secret) = get_secrets()?;

    let google_client_id = ClientId::new(client_id);
    let google_client_secret = ClientSecret::new(client_secret);

    let url = Url::parse("https://accounts.google.com/o/oauth2/v2/auth")?;
    let auth_url = AuthUrl::new(url);

    let url = Url::parse("https://www.googleapis.com/oauth2/v3/token")?;
    let token_url = TokenUrl::new(url);

    let url = Url::parse("http://localhost:8000")?;
    let redirect_url = RedirectUrl::new(url);

    let document_scope =
        Scope::new("https://www.googleapis.com/auth/documents".to_string());
    let drive_scope =
        Scope::new("https://www.googleapis.com/auth/drive".to_string());

    let client = BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
        )
        .add_scope(document_scope)
        .add_scope(drive_scope)
        .set_redirect_url(redirect_url);

    Ok(client)
}

fn extract_from_url<'a>(url: &'a Url, key: &'static str) -> String {
    let code_pair = url
        .query_pairs()
        .find(|pair| {
            let &(ref k, _) = pair;
            k == key
        })
    .unwrap();

    let (_, value) = code_pair;
    value.into_owned()
}

fn create_redirect_server() -> Result<String> {
    let client = create_client()?;
    let (authorize_url, csrf_state) =
        client.authorize_url(CsrfToken::new_random);
    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
        );

    let listener = TcpListener::bind("127.0.0.1:8000")?;

    let (mut socket, _addr) = listener.accept()?;
    let mut reader = BufReader::new(&socket);
    let mut request_line = String::new();

    reader.read_line(&mut request_line)?;
    //println!("request_line: {:?}", request_line);

    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    let url = Url::parse(&("http://localhost".to_string() + redirect_url))?;

    let value = extract_from_url(&url, "code");
    let code = AuthorizationCode::new(value);

    let value = extract_from_url(&url, "state");
    let state = CsrfToken::new(value);

    let message = "Go back to your terminal :)";
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
        message.len(),
        message
        );

    socket.write_all(response.as_bytes())?;

    assert_eq!(state.secret(), csrf_state.secret());
    let token = client.exchange_code(code).unwrap();

    cache(&token)?;

    Ok(token.access_token().secret().to_owned())
}

fn cache(token: &BasicTokenResponse) -> Result<()> {
    let serialized_json = serde_json::to_string(token)?;
    let mut file = File::create("token.json")?;
    file.write_all(serialized_json.as_bytes())?;
    Ok(())
}

fn check_cache() -> Result<String> {
    let mut file = File::open("token.json")?;
    let mut serialized_json = String::new();

    file.read_to_string(&mut serialized_json)?;
    let token: BasicTokenResponse = serde_json::from_str(&serialized_json)?;

    let refresh_token = token.refresh_token().unwrap();
    let mut fresh_token = create_client()?
        .exchange_refresh_token(refresh_token)
        .unwrap();


    if fresh_token.access_token().secret() != token.access_token().secret() {
        //https://github.com/ramosbugs/oauth2-rs/issues/62
        fresh_token.set_refresh_token(Some(refresh_token.to_owned()));
        cache(&fresh_token)?;
        Ok(fresh_token.access_token().secret().to_owned())
    } else {
        Ok(token.access_token().secret().to_owned())
    }
}

pub fn get() -> Result<String> {
    check_cache().or_else(|_err|{
        create_redirect_server()
    })
}
