// rust reqwest get json
use reqwest;
use serde_json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // KNOWN RETURN - HashMap
    const URL2: &str = "https://httpbin.org/ip";
    match reqwest::get(URL2).await {
        Ok(resp) => {
            let json = resp.json::<HashMap<String, String>>().await?;
            println!("{:?}", json)
        }
        Err(err) => {
            println!("Reqwest Error: {}", err)
        }
    }

    // UNKNOWN RETURN - serde_json::Value
    const URL1: &str = "https://dummyjson.com/products/1";
    match reqwest::get(URL1).await {
        Ok(resp) => {
            let json: serde_json::Value = resp.json().await?;
            println!("{:?}", json)
        }
        Err(err) => {
            println!("Reqwest Error: {}", err)
        }
    }

    Ok(())
}
