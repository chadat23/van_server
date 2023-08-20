use std::process::Command;
use std::collections::HashMap;
use futures::future::join_all;

use regex::Regex;
use reqwest;

use instructions::Device;

#[tokio::main]
async fn main() {
    // let nmap_output = run_nmap();
    // let ips = extract_ips(&nmap_output);
    // for ip in ips {
    //     println!("{}", ip);
    // }

    let devices = get_devices().await;

    println!("Devices!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    for (d, s) in devices.iter() {
        println!("    Device: {}, IP: {}", (*d).to_str(), s);
    }

}

async fn get_devices() -> HashMap<Device, String> {
    let ips = get_ips();
    dbg!(&ips);

    let mut devices = HashMap::new();

    let futures: Vec<_> = ips.into_iter().map(|ip| tokio::spawn(get_device(ip))).collect();

    let results: Vec<_> = join_all(futures).await;

    for result in results {
        match result {
            Ok(Some((device, ip))) => {
                devices.insert(device, ip);
            }
            _ => {}
        }
    }

    devices
}

async fn get_device(ip: String) -> Option<(Device, String)> {
    let url = format!("http://{}/device", ip);
    dbg!(&url);
    let body = match reqwest::get(&url).await {
        Ok(response) => match response.text().await {
            Ok(text) => {
                println!("--------------------------- Returned thing: {}\n{}\n", &url, &text);
                text
            },
            Err(_) => return None,
        },
        Err(_) => return None,
    };
    dbg!(&body);

    Some((Device::RoofVent, ip))
}

fn run_nmap() -> String {
    let output = Command::new("nmap")
        .arg("-sn")
        .arg("192.168.1.0/24")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).into_owned()
    } else {
        panic!("Command failed: {}", String::from_utf8_lossy(&output.stderr));
    }
}

fn extract_ips(nmap_output: &str) -> Vec<String> {
    let ip_regex = Regex::new(r"\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b").unwrap();
    ip_regex
        .find_iter(nmap_output)
        .map(|match_| match_.as_str().to_string())
        .collect()
}

fn get_ips() -> Vec<String> {
    let nmap_output = run_nmap();
    extract_ips(&nmap_output)
}
