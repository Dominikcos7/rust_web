use std::{collections::HashMap, fs};

fn main() -> std::io::Result<()> {
    let config = fs::read_to_string("config.txt").expect("Should be able to read config file");
    let config = config.trim();
    let config_lines = config.split("\r\n").collect::<Vec<_>>();
    let mut config: HashMap<String, String> = HashMap::new();
    for line in config_lines {
        let (k, v) = line.split_once("=").expect("Config line should contain an equal sign (=).");
        config.insert(k.trim().into(), v.trim().into());
    }

    let host = config.get("host").expect("Config should include host.");
    let listener = std::net::TcpListener::bind(host)?;

    for stream in listener.incoming() {
        webserver::handle_client(stream?);
    }

    Ok(())
}