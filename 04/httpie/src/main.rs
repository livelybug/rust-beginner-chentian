use clap::{Parser, Subcommand};
use reqwest::Url;
use anyhow::{Result, Context};
use std::str::FromStr;

#[derive(Debug, Subcommand)]
enum MyHttpSubcommands {
    Post {
        #[arg(value_parser = validate_url)]
        url: Url,
        body: Vec<String>,
    },
}

#[derive(Debug, Parser)]
#[command(name = "MY_HTTP_CMD")]
struct MyHttpCmd {
    #[command(subcommand)]
    command: MyHttpSubcommands,
}

fn validate_url(url: &str) -> Result<Url> {
    let _url: Url = url.parse()?;    
    // Ok(url.into())
    Ok(_url)
}

fn validate_body(body: &Vec<String>) -> Result<()> {
    for item in body {
        if !item.contains('=') {
            return Err(anyhow::anyhow!("Invalid body item: {}", item));
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = MyHttpCmd::parse();
    println!("{:?}", args);
/* 
    match args.command {
        MyHttpSubcommands::Post { url, body } => {
            let parsed_url = validate_url(&url)?;
            validate_body(&body)?;

            println!("URL: {}", parsed_url);
            println!("Body: {:?}", body);
        }
    }
 */
    Ok(())
}
