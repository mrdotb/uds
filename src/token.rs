extern crate base64;
extern crate oauth2;
extern crate rand;
extern crate url;
extern crate serde_json;

use oauth2::basic::BasicClient;
use oauth2::basic::BasicTokenResponse;
use oauth2::*;
use oauth2::prelude::*;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl, StandardTokenResponse,
};
use std::env;
use std::io::{BufRead, BufReader, Write, Error};
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpListener;
use url::Url;

fn create_client() -> BasicClient {
    let google_client_id = ClientId::new(
        env::var("GOOGLE_CLIENT_ID").expect("Missing the GOOGLE_CLIENT_ID environment variable."),
        );
    let google_client_secret = ClientSecret::new(
        env::var("GOOGLE_CLIENT_SECRET")
        .expect("Missing the GOOGLE_CLIENT_SECRET environment variable."),
        );
    let auth_url = AuthUrl::new(
        Url::parse("https://accounts.google.com/o/oauth2/v2/auth")
        .expect("Invalid authorization endpoint URL"),
        );
    let token_url = TokenUrl::new(
        Url::parse("https://www.googleapis.com/oauth2/v3/token")
        .expect("Invalid token endpoint URL"),
        );

    BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
        )
        // This example is requesting access to the "documents".
        .add_scope(Scope::new(
                "https://www.googleapis.com/auth/documents".to_string(),
                ))
        // This example will be running its own server at localhost:8080.
        // See below for the server implementation.
        .set_redirect_url(RedirectUrl::new(
                Url::parse("http://localhost:8080").expect("Invalid redirect URL"),
                ))
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

fn create_redirect_server(client: BasicClient) -> () {
    let (authorize_url, csrf_state) = client.authorize_url(CsrfToken::new_random);
    println!(
        "Open this URL in your browser:\n{}\n",
        authorize_url.to_string()
        );

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let (mut socket, _addr) = listener.accept().unwrap();
    let mut reader = BufReader::new(&socket);
    let mut request_line = String::new();

    reader.read_line(&mut request_line).unwrap();
    println!("request_line: {:?}", request_line);

    let redirect_url = request_line.split_whitespace().nth(1).unwrap();
    let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

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

    socket.write_all(response.as_bytes()).unwrap();

    println!("Google returned the following code:\n{}\n", code.secret());
    println!(
        "Google returned the following state:\n{} (expected `{}`)\n",
        state.secret(),
        csrf_state.secret()
        );

    let token = client.exchange_code(code).unwrap();

    // Exchange the code with a token.
    //println!("secret: {}", token.access_token().secret());

    let serialized_json = serde_json::to_string(&token).unwrap();
    println!("{}", serialized_json);

    // The server will terminate itself after collecting the first code.
    //client.exchange_code(code).unwrap();

    //println!("token: {}", token.access_token().secret());
    let mut file = File::create("token.json").unwrap();
    file.write_all(serialized_json.as_bytes()).expect("Unable to write data");
}

pub fn get() -> String {
    let mut file = File::open("token.json").unwrap();
    let mut serialized_json = String::new();
    file.read_to_string(&mut serialized_json).unwrap();
    let token: BasicTokenResponse = serde_json::from_str(&serialized_json).unwrap();
    //println!("token: {}", token.access_token().secret());
    token.access_token().secret().clone()
    //let client = create_client();
    //create_redirect_server(client);
}
