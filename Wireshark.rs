use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.csv");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut ip_map: HashMap<String, Vec<(String, String)>> = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() != 5 {
            continue;
        }
        let timestamp = parts[0].trim().to_string();
        let src_ip = parts[1].trim().to_string();
        let dst_ip = parts[2].trim().to_string();
        let protocol = parts[3].trim().to_string();
        let port = parts[4].trim().to_string();

        let entry = ip_map.entry(src_ip.clone()).or_insert(Vec::new());
        entry.push((timestamp, protocol.clone()));

        let entry = ip_map.entry(dst_ip.clone()).or_insert(Vec::new());
        entry.push((timestamp, protocol.clone()));
    }

    for (ip, data) in ip_map {
        println!("IP: {}", ip);
        println!("Data: {:?}", data);
    }

    Ok(())
}