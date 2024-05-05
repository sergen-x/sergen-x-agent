use reqwest::{Error, Response, Client, Url};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;
use std::fs::File;
use std::io;
use std::io::Error as IoError;

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
    let resp: Response = client.get(url).send().await?;
    if !resp.status().is_success() {
        eprintln!("HTTP request failed: Code {}", resp.status());
        return Ok(());
    }

    let filename_result: Result<String, io::Error> = extract_filename(&resp, url);
    let filename: String = match filename_result {
        Ok(name) => name,
        Err(err) => {
            eprintln!("Failed to extract filename: {}", err);
            return Ok(());
        }
    };

    println!("Downloading file: {}", filename);
    // Read response body and write to file
    let body = resp.text().await?;
    let downloaded_file_name: String = filename.clone();
    let mut out: File = File::create(filename).expect("err creating file");
    io::copy(&mut body.as_bytes(), &mut out).expect("err copying client");
    println!("File downloaded successfully: {}", downloaded_file_name);

    Ok(())
}

// Todo: Pass in a default filename, eg server.jar
fn extract_filename(resp: &Response, url: &str) -> Result<String, IoError> {
    if let Some(name) = extract_filename_from_response(resp) {
        Ok(name)
    } else if let Some(name) = extract_filename_from_url(url) {
        Ok(name)
    } else {
        Err(IoError::new(io::ErrorKind::Other, "Error: Filename not found in response"))
    }
}

// https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition
fn extract_filename_from_response(resp: &reqwest::Response) -> Option<String> {
    if let Some(content_disposition) = resp.headers().get("Content-Disposition") {
        content_disposition
            .to_str()
            .ok()
            .and_then(|content|
                parse_filename_from_content_disposition(content)
            ).map(|filename| filename.to_string())
    }
    else {
        None
    }
}

fn parse_filename_from_content_disposition(content: &str) -> Option<&str> {
    if let Some(start) = content.find("filename=") {
        let filename: &str = &content[start + 9..];
        let filename: &str = filename.trim_matches(|c| c == '"' || c == '\\');
        if !filename.is_empty() {
            return Some(filename);
        }
    }
    None
}

fn extract_filename_from_url(url: &str) -> Option<String> {
    Url::parse(url)
        .ok()
        .and_then(|parsed_url| {
            parsed_url
                .path_segments()
                .and_then(|segments| segments.last().map(ToString::to_string))
        })
}