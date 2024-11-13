// Curl Utility
use reqwest;
use clap::ArgMatches;
use clap::{Arg, Command};

fn main() {
    let matches = get_arguments();
    if let Some(website) = matches.get_one::<String>("Host") {
        request(website);
    }
}

#[tokio::main]
async fn request(server : &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(server).await?;
    
    let body = response.text().await?;
    println!("Requsting URL: {}", server);
    println!("Method: GET");
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
        // .arg(
        //     Arg::new("")
        //         .short("")
        //         .action(clap::ArgAction::SetTrue)
        //         .help("Case-insensitive search")
        //         .required(false),
        // )
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