use reqwest::{Error, Response, Client};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io::BufWriter;
use std::io;
use tokio;

static CLIENT: Lazy<Client> = Lazy::new(|| Client::new());

pub async fn get<T>(url: &str) -> Result<T, Error>
where
    T: DeserializeOwned, 
{
    let client: &Client = &*CLIENT;
    // TODO: check http status code
    let response: Response = client.get(url).send().await?;
    let body: T = response.json::<T>().await?;

    Ok(body)
}

pub async fn download_file(url: &str) -> Result<(), Error> {
    let client: &Client = &*CLIENT;
    // TODO: check http status code
    println!("sending request");
    let resp = client.get(url).send().await?;
    let filename_opt = extract_filename_from_response(&resp);

    if resp.status().is_success() {
        match filename_opt {
            Some(filename) => {
                let body = resp.text().await.expect("err decoding body");
                let mut out = File::create(filename).expect("err creating file");
                io::copy(&mut body.as_bytes(), &mut out).expect("err copying client");
            }
            None => {
                eprintln!("Error: Filename not found in response");
            }
        }
    } else {
        println!("HTTP request failed: Code {}", resp.status());
    }
    Ok(())
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition
fn extract_filename_from_response(resp: &reqwest::Response) -> Option<String> {
    if let Some(content_disposition) = resp.headers().get("Content-Disposition") {
        if let Ok(content) = content_disposition.to_str() {
            if let Some(filename) = parse_filename_from_content_disposition(content) {
                return Some(filename.to_string());
            }
        }
    }
    None
}

fn parse_filename_from_content_disposition(content: &str) -> Option<&str> {
    if let Some(start) = content.find("filename=") {
        let filename = &content[start + 9..];
        let filename = filename.trim_matches(|c| c == '"' || c == '\\');
        if !filename.is_empty() {
            return Some(filename);
        }
    }
    None
}