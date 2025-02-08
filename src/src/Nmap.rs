#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::fs;
use std::io::Read;
//use serde_json;
use serde;
use serde_json::{Result,Value};
use xml2json_rs::JsonBuilder;
//use std::error::Error;


pub fn nmapMapping ()->Vec<entry>{
    //Parse Files 
    println!("\n --------------------------------------------------------------------------- \nEnter the filenames of the Nmap files you want to import (separated by spaces): \n --------------------------------------------------------------------------- \n");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let filenames: Vec<&str> = input.trim().split_whitespace().collect();

    // Build JSON Files
    let entries = Vec::new();
    println!("\n --------------------------- \nConverting .XML file to .JSON \n ---------------------------\n");
    //println!("{:?}", filenames);
    for filename in filenames {
        //let scan = entry::default();
        let mut file = fs::File::open(filename).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

            let json_builder = JsonBuilder::default();
            let json = json_builder.build_pretty_string_from_xml(&contents).unwrap();
            println!("{}", json);
            //fn untyped_example() -> Result<()> {
                // Parse the string of data into serde_json::Value.
                //let v: Value = serde_json::from_str(&json).unwrap()?;
            
                // Access parts of the data by indexing with square brackets.
                //println!("{}{},", v["addr"], v["addrtype"]);
            
                //Ok(())
                //}
            
        //loop {
            //let host = value.get("nmaprun").unwrap().get("host").unwrap();
        

        //}

    };
    println!(" \n--------------\nFile Converted! \n-------------- ");
    //Print Out
    for file in &entries {
        println!("{:?}", file);
    } 

    
    return entries;

}

#[derive(Default, Debug)]
pub struct entry {
    pub entries: Vec<entryData>,
    pub ip: String,
}
#[derive(Default, Debug)]
pub struct entryData {
    pub port: String,
    pub state: String,
    pub service: String,
}

////////////////////////////////////////////////////////////
/// Serde Functions
  
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Host {
    pub address: Address,
    pub ports: PortScan,
}


#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Address {
    #[serde(alias = "addr")]
    addr: String,
    #[serde(alias = "addrtype")]
    addrtype: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct PortScan {
    extraports: ExtraPorts,
    #[serde(alias = "port")]
    ports: Vec<Port>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ExtraPorts {
    #[serde(alias = "count")]
    count: String,
    #[serde(alias = "state")]
    state: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Port {
    state: State,
    service: Service,
    #[serde(alias = "portid")]
    port: String,
    #[serde(alias = "protocol")]
    protocol: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Service {
    #[serde(alias = "name")]
    name: String,
    #[serde(alias = "conf")]
    conf: String,
    #[serde(alias = "method")]
    method: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct State {
    #[serde(alias = "state")]
    state: String,
    #[serde(alias = "reason")]
    reason: String,
    #[serde(alias = "reason_ttl")]
    reason_ttl: String,
}


