use reqwest::Client;

pub async fn check_url_status_code(client: &Client, url: &str) -> Result<u16, reqwest::Error> {
    let res = client.get(url).send().await?;
    Ok(res.status().as_u16())
}