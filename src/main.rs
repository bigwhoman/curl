// Curl Utility
use clap::ArgMatches;
use clap::{Arg, Command};
use reqwest;
fn main() {
    let matches = get_arguments();
    if let Some(website) = matches.get_one::<String>("Host") {
        if let Some(method) = matches.get_one::<String>("Method") {
            
                // TODO :
                /*
                 *   Handle Valid Base Protocol (HTTP -- HTTPS)
                 *   Invalid IP Address
                 */
                let data : Option<String> = matches.get_one::<String>("Data").map(|s| s.to_string());
                match request(website, method, data) {
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
    println!();
}

#[tokio::main]
async fn request(
    server: &str,
    method: &str,
    data: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Requsting URL: {}", server);
    println!("Method: {}", method);
    let response;
    if method == "GET" {
        response = reqwest::get(server).await?;
    } else {
        response = reqwest::Client::new()
            .post(server)
            .body(data.unwrap().to_string())
            .send()
            .await?;
    }
    let status = response.status();
    let body = response.text().await?;
    if !status.is_success() {
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
                .default_value("GET"),
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
