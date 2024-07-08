use clap::{Parser, Subcommand};
use reqwest::{Url, Client, Response};
use anyhow::{anyhow, Result, Context};
use std::str::FromStr;

#[derive(Debug, Subcommand)]
enum MyHttpSubcommands {
    Post(Post),
    Get(Get)
}

#[derive(Parser, Debug)]
struct Get {
    #[arg(value_parser = validate_url)]
    url: String,
}

#[derive(Parser, Debug)]
struct Post {
    #[arg(value_parser = validate_url)]
    url: String,
    #[arg(value_parser = parse_kv_pair)]
    body: Vec<KvPair>,
}

#[derive(Debug, Parser)]
#[command(name = "MY_HTTP_CMD")]
struct MyHttpCmd {
    #[command(subcommand)]
    command: MyHttpSubcommands,
}

fn validate_url(url: &str) -> Result<String> {
    let _url: Url = url.parse()?;    
    Ok(url.into())
}

// fn validate_body(body: &Vec<String>) -> Result<()> {
//     for item in body {
//         if !item.contains('=') {
//             return Err(anyhow::anyhow!("Invalid body item: {}", item));
//         }
//     }
//     Ok(())
// }

/// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构
#[derive(Debug, PartialEq, Clone)]
struct KvPair {
    k: String,
    v: String,
}

/// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 使用 = 进行 split，这会得到一个迭代器
        let mut split = s.split('=');
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 从迭代器中取第一个结果作为 key，迭代器返回 Some(T)/None
            // 我们将其转换成 Ok(T)/Err(E)，然后用 ? 处理错误
            k: (split.next().ok_or_else(err)?).to_string(),
            // 从迭代器中取第二个结果作为 value
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

/// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

async fn get(client: Client, args: &Get) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    print!("{:?}", resp);
    Ok(())
}

async fn post(client: Client, args: &Post) -> Result<()> {
    let resp = client.get(&args.url).send().await?;
    print!("{:?}", resp);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new();
    
    let my_claps = MyHttpCmd::parse();
    println!("{:?}", my_claps);
    let result = match my_claps.command {
        MyHttpSubcommands::Get(ref args) => get(client, args).await?,
        MyHttpSubcommands::Post(ref args) => post(client, args).await?,
    };

    Ok(())
}
