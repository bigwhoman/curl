use std::fmt::format;

// Curl Utility
use clap::{Arg, Command};
use clap::{ArgMatches};
use reqwest;

use serde_json::{Value};
fn main() {
    let matches = get_arguments();
    let json_value;
    let mut is_json = false; 
    if let Some(website) = matches.get_one::<String>("Host") {
        let method = if matches.contains_id("Json") {
            is_json = true;
            "POST"
        } else {
            matches.get_one::<String>("Method").unwrap()
        };
        /*
         *   Handle Valid Base Protocol (HTTP -- HTTPS)
         *   Invalid IP Address
         */

         let raw_data: Option<String> = if matches.contains_id("Json") {
            matches.get_one::<String>("Json").map(|s| s.to_string())
        } else if matches.contains_id("Data") {
            Some(matches.get_one::<String>("Data").unwrap().to_string())
        } else {None};

        let data: Option<String> = if matches.contains_id("Json") {
            matches.get_one::<String>("Json").map(|s| s.to_string())
        } else if matches.contains_id("Data") {
            Some(format(format_args!(
                "{{ {} }}",
                matches.get_one::<String>("Data").unwrap().split("&")
                    .map(|x| {
                        let inp: Vec<String> = x
                            .split("=")
                            .map(|l| {
                                return format(format_args!("\"{}\"", l));
                            })
                            .collect();
                        let googoo = inp.join(":");
                        return googoo;
                    })
                    .collect::<Vec<String>>()
                    .join(",")
            )))
        } else {None};

        match data {
            None => {
                json_value = serde_json::from_str("{}").unwrap();
            }
            Some(data) => {
                json_value = serde_json::from_str(&data).unwrap();
            }
        }
        match request(website, method, Some(json_value), raw_data ,is_json) {
            Ok(()) => (),
            Err(e) => {
                print!("Error: ");
                println!("{}", e);
                let message = e
                    .source()
                    .map(|src| src.to_string())
                    .unwrap_or_else(|| e.to_string());
                // println!("------> {}", message);
                if message.contains("invalid IPv") {
                    println!("The URL contains an {}.", message);
                } else if message.contains("URL scheme is not allowed")
                    || message.contains("relative URL without a base")
                {
                    println!("The URL does not have a valid base protocol.");
                } else if message.contains("invalid port number") {
                    println!("The URL contains an invalid port number.");
                } else if message.contains("error trying to connect") {
                    println!("Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");
                } else if message.contains("Request failed with status code:") {
                    println!("{}", message);
                }
            }
        };
    }
}

#[tokio::main]
async fn request(
    server: &str,
    method: &str,
    json_data: Option<Value>,
    raw_data : Option<String>,
    is_json : bool
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Requsting URL: {}", server);
    println!("Method: {}", method);
    let response;
    if method == "GET" {
        response = reqwest::get(server).await?;
    } else {
        if is_json {
            println!("JSON: {}", raw_data.unwrap());
        } else {
            println!("Data: {}",raw_data.unwrap());
        }
        response = reqwest::Client::new()
            .post(server)
            .header("Content-Type", "application/json")
            .json(&json_data) // converts the JSON value to the request body
            .send()
            .await?;
    }
    let status = response.status();
    let body = response.text().await?;

    if !status.is_success() {
        return Err(format!("Request failed with status code: {}.", status.as_u16()).into());
    }
    if serde_json::from_str::<serde_json::Value>(&body).is_ok() {
        let parsed: Value = serde_json::from_str(&body).unwrap();
        let pretty = serde_json::to_string_pretty(&parsed);
        println!("Response body (JSON with sorted keys):\n{}", pretty.unwrap());
    } else {
        println!("Response body:\n{}", body);
    }

    Ok(())
}

fn get_arguments() -> ArgMatches {
    let matches = Command::new("Curl")
        .version("1.0")
        .author("Hooman Keshvari")
        .about("Curl command line utility")
        .arg(
            Arg::new("Json")
                .long("json")
                .help("Post a json")
                .num_args(1)
                .required(false),
        )
        .arg(
            Arg::new("Data")
                .short('d')
                .help("Data to send in post")
                .required(false),
        )
        .arg(
            Arg::new("Method")
                .short('X')
                .long("method")
                .help("Http method")
                .required(false)
                .value_parser(["GET", "POST"])
                .default_value("GET")
                .requires("Data"),
        )
        .arg(
            Arg::new("Host")
                .help("Hostname or IP to send the request to")
                .value_name("Host")
                .required(true)
                .num_args(1)
                .index(1),
        )
        .get_matches();
    return matches;
}
