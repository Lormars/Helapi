use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Map, Value};
use std::collections::HashMap;
use std::io::Read;

mod error;

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    method: String,
    content_type: Option<String>,
    target: String,
    body: Option<Value>,
}

fn decouple_value(map: &Map<String, Value>) -> HashMap<String, String> {
    let form_data: HashMap<String, String> = map
        .iter()
        .map(|(k, v)| (k.to_owned(), v.to_string()))
        .collect();
    form_data
}

async fn send_request(client: &Client, request: &Request) -> Result<Response, error::MyError> {
    match request.method.to_lowercase().as_str() {
        "get" => Ok(client.get(&request.target).send().await?),
        "post" | "put" => {
            let method = request.method.to_lowercase();
            match request
                .content_type
                .as_ref()
                .expect("content type not found")
                .to_lowercase()
                .as_str()
            {
                "application/json" => Ok(if method == "post" {
                    client.post(&request.target)
                } else {
                    client.put(&request.target)
                }
                .json(&request.body)
                .send()
                .await?),
                "x-www-form-urlencoded" => {
                    if let Value::Object(ref map) = request.body.as_ref().expect("body not found") {
                        let form_data = decouple_value(map);

                        Ok(if method == "post" {
                            client.post(&request.target)
                        } else {
                            client.put(&request.target)
                        }
                        .form(&form_data)
                        .send()
                        .await?)
                    } else {
                        Err(error::MyError::Syntax("Invalid body".to_string()))
                    }
                }
                _ => Err(error::MyError::Syntax("Invalid content type".to_string())),
            }
        }

        _ => Err(error::MyError::Syntax(
            "Invalid method. Only support GET, POST, and PUT".to_string(),
        )),
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

fn template() -> String {
    r#"{
	"target": "http://localhost:5555/books",
	"method": "post",
	"content_type": "application/json",
	"body": 
		{
		    "title": "ndss",
			"author": "adsfsadf",
			"publishYear": "1234"
		 }

	
}

{
	"target": "http://localhost:3000/post/urlform",
	"content_type": "x-www-form-urlencoded",
	"method": "post",
	"body": {
		"name": "hello",
		"age": 3
	}
		
}
"#
    .to_string()
}

#[tokio::main]
async fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Stdin read error");

    if input.is_empty() {
        let temp = template();
        println!("{temp}");
        return;
    }

    let v: Request = serde_json::from_str(&input).expect("Wrong JSON Format!");
    // dbg!(&v);

    let client = reqwest::Client::new();

    println!("{input}");

    match send_request(&client, &v).await {
        Ok(res) => handle_response(res).await,
        Err(e) => println!("Request error: {:?}", e),
    }
}
