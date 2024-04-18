use reqwest::{
    header, header::HeaderMap, header::HeaderName, header::HeaderValue, header::AUTHORIZATION,
    Client, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::{to_string_pretty, Map, Value};
use std::collections::HashMap;
use std::env;
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
    let status = res.status();
    let text_result = res.text().await;

    println!("----------result----------\n");
    println!("Response status: {}\n", status);

    match text_result {
        Ok(text) => match serde_json::from_str::<Value>(&text) {
            Ok(p) => match to_string_pretty(&p) {
                Ok(pretty) => println!("{}", pretty),
                Err(_) => println!("{text}"),
            },
            Err(_) => println!("{text}"),
        },
        Err(e) => println!("Error: {:?}", e),
    }
    println!("----------result----------");
}

fn template() -> String {
    r#"{
	"target": "https://simple-books-api.glitch.me/orders",
	"method": "post",
	"content_type": "application/json",
	"authorization": "Bearer 2586943e89d9fee22379e16ec81470ac9a17292fa155a96d9a98be7da7412c74",
	"headers": {
		"foo": "bar",
		"foo2": "bar2"
	},
	"body": 
		{
		    "bookId": "1",
			"clientEmail": "ndss"
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

fn react_template() -> String {
    r#"import React from 'react'

const page = () => {
    return (
        <div>page</div>
    )
}

export default page
    "#.to_string()
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

fn remove_control_characters(s: &str) -> String {
    s.chars()
     .filter(|c| !c.is_control())
     .collect()
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    //check flag

    let mut t_flag = false; //api template
    let mut r_flag = false; //react template

    for arg in args.iter() {
        match arg.as_str() {
            "-t" => t_flag = true,
            "-r" => r_flag = true,
             _   => {}
        }
    }

    if t_flag {
        let temp = template();
        println!("{temp}");
        return;
    } else if r_flag {
        let temp = react_template();
        println!("{temp}");
        return;
    }

    //take input

    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Stdin read error");

    let cleaned_input = remove_control_characters(input.as_str());
    let v: Request = serde_json::from_str(&cleaned_input).expect("Wrong JSON Format!");

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
