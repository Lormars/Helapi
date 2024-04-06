use reqwest::{
    header, header::HeaderMap, header::HeaderName, header::HeaderValue, header::AUTHORIZATION,
    Client, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Map, Value};
use std::collections::HashMap;
use std::io::Read;

mod error;

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    method: String,
    content_type: Option<String>,
    authorization: Option<String>,
    headers: Option<Value>,
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
        "delete" => Ok(client.delete(&request.target).send().await?),

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

fn add_headers(v: &Request, headers: &mut HeaderMap) {
    match v.headers.as_ref() {
        Some(header_json) => {
            let header_map: HashMap<String, String> =
                decouple_value(header_json.as_object().expect("Header value wrong"));
            for (key, value) in header_map {
                let header_name =
                    HeaderName::from_bytes(key.as_bytes()).expect("Invalid header name");
                let header_value =
                    HeaderValue::from_bytes(value.as_bytes()).expect("Invalid header value");
                headers.insert(header_name, header_value);
            }
        }
        None => {}
    }
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
    

    let mut headers = header::HeaderMap::new();

    //add authorization
    if let Some(token) = v.authorization.as_ref() {
        headers.insert(AUTHORIZATION, token.parse().expect("token error"));
    }

    //add headers
    add_headers(&v, &mut headers);

    //build client
    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .expect("client build error");

    //print original request
    println!("{input}");

    match send_request(&client, &v).await {
        Ok(res) => handle_response(res).await,
        Err(e) => println!("Request error: {:?}", e),
    }
}
