extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate url;
extern crate serde_json;
//extern crate failure;


use crate::errors::*;
use serde_json::{Value};
use oauth2::basic::BasicClient;
use oauth2::basic::BasicTokenResponse;
use oauth2::*;
use oauth2::prelude::*;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl, 
    //StandardTokenResponse, RequestTokenError
};
use std::env;
use std::io;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;
use std::net::TcpListener;
//use std::option::{NoneError};
use url::{Url};


fn create_client() -> Result<BasicClient> {
    let mut file = File::open("credentials.json")?;
    let mut serialized_json = String::new();

    file.read_to_string(&mut serialized_json)?;
    let credentials: Value = serde_json::from_str(&serialized_json)?;
    println!("credentials {:?}", credentials);


    let var_client_id = env::var("GOOGLE_CLIENT_ID")?;
    let var_client_secret = env::var("GOOGLE_CLIENT_SECRET")?;

    let google_client_id = ClientId::new(var_client_id);
    let google_client_secret = ClientSecret::new(var_client_secret);

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

fn create_redirect_server() -> Result<BasicTokenResponse> {
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
    println!("request_line: {:?}", request_line);

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
    let token = cache(token)?;

    Ok(token)
}

fn cache(token: BasicTokenResponse) -> Result<BasicTokenResponse> {
    // Save serialized token
    let serialized_json = serde_json::to_string(&token)?;
    let mut file = File::create("token.json")?;
    file.write_all(serialized_json.as_bytes())?;
    Ok(token)
}

fn check_cache() -> Result<BasicTokenResponse> {
    let mut file = File::open("token.json")?;
    let mut serialized_json = String::new();

    file.read_to_string(&mut serialized_json)?;
    let token: BasicTokenResponse = serde_json::from_str(&serialized_json)?;

    let refresh_token = token.refresh_token().unwrap();
    let mut fresh_token = create_client()?
        .exchange_refresh_token(refresh_token)
        .unwrap();


    if fresh_token.access_token().secret() != token.access_token().secret() {
        fresh_token.set_refresh_token(Some(refresh_token.to_owned()));
        cache(fresh_token)
    } else {
        Ok(token)
    }
}

pub fn get() -> Result<BasicTokenResponse> {
    //check_cache().or_else(|_err|{
        create_redirect_server()
    //})
}
