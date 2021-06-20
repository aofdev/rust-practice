mod errors;
use chrono::DateTime;
use errors::MyCustomError;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::Deserialize;
use std::collections::HashMap;
use std::{env, fs};
fn main() {
    // Unrecoverable Errors
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    // Recoverable Errors
    // We can use unwrap as before or use expect - it’s same as unwrap but lets us add extra error message.
    let content = fs::read_to_string("./Cargo.toml").expect("Can't read Cargo.toml");
    println!("{}", content);

    // Use a fallback value
    // unwrap_or, unwrap_or_else, unwrap_or_default
    let port = env::var("PORT").unwrap_or("8080".to_string());
    println!("Port: {}", port);

    //Bubble up the error
    match get_current_ip() {
        Ok(my_ip) => println!("My IP: {}", my_ip),
        Err(e) => eprintln!("Error get_current_ip:( \n  {}", e),
    }

    // Match boxed errors
    match get_github_my_user_updated() {
        Ok(date) => println!("My github updated: {}", date),
        Err(e) => {
            eprintln!("Error get_github_my_user_updated:(");
            if let Some(err) = e.downcast_ref::<reqwest::Error>() {
                eprintln!("Request Error: {}", err)
            } else if let Some(err) = e.downcast_ref::<chrono::format::ParseError>() {
                eprintln!("Parse Error: {}", err)
            }
        }
    }

    // Match custom errors
    match get_github_my_user_updated_custom_error() {
        Ok(date) => println!("My github updated: {}", date),
        Err(e) => {
            eprintln!("Error get_github_my_user_updated_custom_error:(");
            match e {
                MyCustomError::HttpError => eprintln!("Request Error: {}", e),
                MyCustomError::ParseError => eprintln!("Parse Error: {}", e),
            }
        }
    }
}

// Bubble up the error
fn get_current_ip() -> Result<String, reqwest::Error> {
    let url = "https://httpbin.org/ip";
    let result = reqwest::blocking::get(url);
    let response = match result {
        Ok(res) => res,
        Err(err) => return Err(err),
    };
    let body = response.json::<HashMap<String, String>>();
    let json = match body {
        Ok(json) => json,
        Err(err) => return Err(err),
    };
    let ip = json["origin"].to_string();
    Ok(ip)
}

#[derive(Deserialize, Debug)]
struct GithubUser {
    updated_at: String,
}

// Bubble up multiple errors using the ? operator
fn get_github_my_user_updated() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.github.com/users/aofdev";
    let mut headers = HeaderMap::new();
    // add the user-agent header required by github
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    let resp = reqwest::blocking::Client::new()
        .get(url)
        .headers(headers)
        .send()?
        .json::<GithubUser>()?;

    let parsed_date = DateTime::parse_from_rfc3339(&resp.updated_at)?;
    let date = parsed_date.format("%d %B %Y").to_string();

    Ok(date)
}

// Bubble up custom errors
fn get_github_my_user_updated_custom_error() -> Result<String, MyCustomError> {
    let url = "https://api.github.com/users/aofdev";
    let mut headers = HeaderMap::new();
    // add the user-agent header required by github
    headers.insert(USER_AGENT, HeaderValue::from_static("reqwest"));

    let resp = reqwest::blocking::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .map_err(|_| MyCustomError::HttpError)?
        .json::<GithubUser>()
        .map_err(|_| MyCustomError::HttpError)?;

    let parsed_date =
        DateTime::parse_from_rfc3339(&resp.updated_at).map_err(|_| MyCustomError::ParseError)?;
    let date = parsed_date.format("%d %B %Y").to_string();

    Ok(date)
}
