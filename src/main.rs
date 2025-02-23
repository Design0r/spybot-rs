use anyhow::Result;
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct TSUser {
    name: String,
    channel_id: u32,
}

#[derive(Debug, Deserialize, Serialize)]
struct TSChannel {
    id: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LiveResponse {
    clients: Vec<TSUser>,
    channels: Vec<TSChannel>,
}

type TSTree = Vec<(String, Vec<String>)>;

const BANNER: &str = r#"
     ___ _ __  _   _| |__   ___ | |_           _ __ ___ 
    / __| '_ \| | | | '_ \ / _ \| __|  _____  | '__/ __|
    \__ \ |_) | |_| | |_) | (_) | |_  |_____| | |  \__ \
    |___/ .__/ \__, |_.__/ \___/ \__|         |_|  |___/
        |_|    |___/                                    
"#;

fn print_tree(tree: &TSTree) {
    println!("{}", BANNER.truecolor(166, 53, 25).bold());

    for (channel, clients) in tree {
        if clients.len() > 0 {
            println!("    \u{25BA} {}", channel.green());
        } else {
            println!("    \u{25BA} {}", channel.dimmed());
        }

        for c in clients {
            println!("      \u{2605} {}", c);
        }
    }

    println!();
}

fn escape_ts_symbols(input: &str) -> String {
    let new = input.replace("\\s", " ").replace("\\/", "/");
    new
}

fn parse_tree(data: &LiveResponse) -> TSTree {
    data.channels
        .iter()
        .filter(|channel| !channel.name.starts_with("["))
        .map(|channel| {
            let clients = data
                .clients
                .iter()
                .filter(|client| client.channel_id == channel.id)
                .map(|client| escape_ts_symbols(&client.name))
                .collect();

            return (escape_ts_symbols(&channel.name), clients);
        })
        .collect()
}

fn fetch() -> Result<LiveResponse> {
    let res =
        reqwest::blocking::get("https://spybot.bensge.com/api/v1/live")?.json::<LiveResponse>()?;

    Ok(res)
}

fn main() {
    let res = fetch().expect("Failed to fetch data");
    let tree = parse_tree(&res);
    let _ = print_tree(&tree);
}
