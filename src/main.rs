use clap::Parser;
use anyhow::{anyhow, Result};
use std::str::FromStr;
use reqwest::Url;
/// A native httpie implementation with Rust.
// 定义 HTTPie 的CLI 的主入口，它包含若干个子命令.
#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "cracker <2278801557@qq.com>")]
struct Opts {
    #[clap(subcommand)]
    subcmd: Subcommand,
}

// 子命令分别对应不同的 HTTP 方法，目前仅支持 get 和 post.
#[derive(Parser, Debug)]
enum Subcommand {
    Get(Get),
    Post(Post),
}

/// feed get with an url and we will retrieve the response for you.
// get 子命令.
#[derive(Parser, Debug)]
struct Get {
    #[clap(parse(try_from_str = parse_url))]
    url: String,
}

/// feed post with an url and optional key=value pairs.
/// We will post the data as JSON, and retrieve the response for you.
// post 子命令. 需要输入一个 URl，和若干个可选的键值对，用于 JSON 数据传递给服务器。
#[derive(Parser, Debug)]
struct Post {
    #[clap(parse(try_from_str=parse_url))]
    url: String,
    #[clap(parse(try_from_str=parse_kv_pair))]
    body: Vec<KvPair>,
}

// 命令行中的 key=value 可以通过 parse_kv_pair 解析成 KvPair 结构.
#[derive(Debug, PartialEq)]
struct KvPair {
    k: String,
    v: String,
}

// 当我们实现 FromStr trait 后，可以用 str.parse() 方法将字符串解析成 KvPair.
impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 以 = 进行split，会得到一个迭代器
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            // 得到迭代器的第一个值，迭代器会返回一个 Some(T)/None.
            // 将其转化为 Ok(T)/Err(E), 然后使用 ？ 处理错误.
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

// 因为我们为 KvPair 实现了 FromStr，这里可以直接 s.parse() 得到 KvPair
fn parse_kv_pair(s: &str) -> Result<KvPair> {
    s.parse()
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;
    Ok(s.into())
}


fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
