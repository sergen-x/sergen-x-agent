use reqwest::{Error, Response, Client};
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;

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