use serde::Deserialize;
use std::collections::HashMap;
use std::io::{self, BufRead};

#[derive(Deserialize)]
struct Config {
    netbox_url: String,
    netbox_token: String,
    netbox_key: Option<String>,
}

#[derive(Deserialize)]
struct NetBoxResponse {
    results: Vec<NetBoxInterface>,
}

#[derive(Deserialize)]
struct NetBoxInterface {
    name: String,
    device: Option<NetBoxDevice>,
}

#[derive(Deserialize)]
struct NetBoxDevice {
    url: String,
    #[serde(rename = "name")]
    _name: String,
}

#[derive(Deserialize)]
struct NetBoxDeviceFull {
    name: String,
    asset_tag: Option<String>,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_dir = dirs::config_dir().ok_or("could not determine config directory")?;
    let config_path = config_dir.join("arpbox").join("config.toml");
    let contents = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("failed to read config at {}: {}", config_path.display(), e))?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

fn lookup_mac(
    client: &reqwest::blocking::Client,
    config: &Config,
    mac: &str,
    cache: &mut HashMap<String, Option<String>>,
) -> Option<String> {
    if let Some(cached) = cache.get(mac) {
        return cached.clone();
    }

    let url = format!(
        "{}/api/dcim/interfaces/?mac_address={}",
        config.netbox_url.trim_end_matches('/'),
        mac
    );

    let auth = match &config.netbox_key {
        Some(key) => format!("Bearer nbt_{}.{}", key, config.netbox_token),
        None => format!("Token {}", config.netbox_token),
    };

    let result = client
        .get(&url)
        .header("Authorization", &auth)
        .header("Accept", "application/json")
        .send()
        .ok()
        .and_then(|resp| resp.json::<NetBoxResponse>().ok())
        .and_then(|body| {
            body.results.into_iter().next().and_then(|iface| {
                iface.device.and_then(|d| {
                    let device: NetBoxDeviceFull = client
                        .get(&d.url)
                        .header("Authorization", &auth)
                        .header("Accept", "application/json")
                        .send()
                        .ok()?
                        .json()
                        .ok()?;
                    let name = match device.asset_tag {
                        Some(tag) => format!("{}:{}:{}", device.name, tag, iface.name),
                        None => format!("{}:{}", device.name, iface.name),
                    };
                    Some(name)
                })
            })
        });

    cache.insert(mac.to_string(), result.clone());
    result
}

fn main() {
    let config = match load_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };

    let client = reqwest::blocking::Client::new();
    let mut cache: HashMap<String, Option<String>> = HashMap::new();
    let stdin = io::stdin();

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => continue,
        };

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            println!("{}", line);
            continue;
        }

        // Validate MAC-like format in second field
        let mac = parts[1];
        if mac.len() < 11 || !mac.contains(':') {
            println!("{}", line);
            continue;
        }

        if let Some(device_name) = lookup_mac(&client, &config, mac, &mut cache) {
            let vendor = if parts.len() >= 3 {
                format!("\t{}", parts[2..].join("\t"))
            } else {
                String::new()
            };
            println!(
                "{}\t{}\t(netbox://{}){}",
                parts[0], mac, device_name, vendor
            );
        } else {
            println!("{}", line);
        }
    }
}
