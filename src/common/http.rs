use reqwest::{Error, Response, Client};

pub async fn get<T>(url: &str) -> Result<T, Error>
where
    T: serde::de::DeserializeOwned, 
{
    // TODO: reuse this client
    let client: Client = reqwest::Client::new();

    // TODO: check http status code
    let response: Response = client.get(url).send().await?;
    let body: T = response.json::<T>().await?;

    Ok(body)
}