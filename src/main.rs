use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Value};
use std::io::Read;
use reqwest::{Client, Error, Response};

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    method: String,
    target: String,
    body: Value,
}

async fn send_request(client: &Client, request: &Request) -> Result<Response, Error> {
    match request.method.to_lowercase().as_str() {
        "get" => client.get(&request.target).send().await,
        "post" => client.post(&request.target).json(&request.body).send().await,
        _ => panic!("Invalid method. Only support GET and POST"),
    }
}

async fn handle_response(res: Response) {
    let text_result = res.text().await;
    match text_result {
        Ok(text) => {
            println!("----------result----------");
            match serde_json::from_str::<Value>(&text) {
                Ok(p) => match to_string_pretty(&p) {
                    Ok(pretty) => println!("{}", pretty),
                    Err(_) => println!("{text}"),
                },
                Err(_) => println!("{text}"),
            }
            println!("----------result----------");
        }
        Err(e) => println!("Error: {:?}", e),
    }
}

#[tokio::main]
async fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();


    let v: Request = serde_json::from_str(&input).unwrap();

    let client = reqwest::Client::new();

    println!("{input}");
    
    match send_request(&client, &v).await {
        Ok(res) => handle_response(res).await,
        Err(e) => println!("Request error: {:?}", e),
    }
}
