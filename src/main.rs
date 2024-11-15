

// Curl Utility
use reqwest;
use clap::ArgMatches;
use clap::{Arg, Command};
use reqwest::StatusCode;
fn main() {
    let matches = get_arguments();
    if let Some(website) = matches.get_one::<String>("Host") {
        // TODO : 
        /*
        *   Handle Valid Base Protocol (HTTP -- HTTPS)
        *   Invalid IP Address
        */
        match request(website) {
            Ok(()) => (),
            Err(e) => {
                print!("Error: ");
                println!("{}",e);
                let message = e.source()
                    .map(|src| src.to_string()) 
                    .unwrap_or_else(|| e.to_string()); 
                // println!("------> {}", message);
                if message.contains("invalid IPv") {
                    println!("The URL contains an {}.",message);
                } else if message.contains("URL scheme is not allowed") || 
                            message.contains("relative URL without a base"){
                    println!("The URL does not have a valid base protocol.");
                } else if message.contains("invalid port number") {
                    println!("The URL contains an invalid port number.");
                } else if message.contains("error trying to connect") {
                    println!("Unable to connect to the server. Perhaps the network is offline or the server hostname cannot be resolved.");
                } else if message.contains("Request failed with status code:") {
                    println!("{}",message);
                }
            }
        };
    }
}

#[tokio::main]
async fn request(server : &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("Requsting URL: {}", server);
    println!("Method: GET");
    let response = reqwest::get(server).await?;
    let status = response.status();
    let body = response.text().await?;
    if !status.is_success(){
        return Err(format!("Request failed with status code: {}.", status.as_u16()).into());
    }
    println!("Response body:\n{}", body);
    Ok(())
}


fn get_arguments() -> ArgMatches {
    let matches = Command::new("Curl")
        .version("1.0")
        .author("Hooman Keshvari")
        .about("Curl command line utility")
        .arg(
            Arg::new("data")
                .short('d')
                .help("Data to send in post")
                .required(false),
        )
        .arg(
            Arg::new("ٓX POST")
                .long("X POST")
                .action(clap::ArgAction::SetTrue)
                .help("Post Data")
                .required(false),
        )
        .arg(
            Arg::new("Host")
                .help("Hostname or IP to send the request to")
                .value_name("files")
                .required(true)
                .num_args(1)
                .index(1),
        )
        .get_matches();
    return matches;
}