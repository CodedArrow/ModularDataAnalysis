#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::fs;
use std::io::Read;
//use std::collections::HashMap;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::name::QName;
#[derive(Default, Debug)]
struct entry {
    entries: Vec<entryData>,
    ip: String,
    //hostUp: bool,
}
#[derive(Default, Debug)]
struct entryData {
    port: String,
    state: String,
    service: String,
}

fn main() {
    //let mut files = Vec::new();
    // 5 8 11 4 13 0 12 4 
    println!("Enter the filenames of the Nmap XML files you want to import (separated by spaces):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let filenames: Vec<&str> = input.trim().split_whitespace().collect();

    // 17 4 0 3 / 5 8 11 4 13 0 12 4 
    let mut entries = Vec::new();
    print!("{:?}", filenames);
    for filename in filenames {
        let mut scan = entry::default();
        let mut file = fs::File::open(filename).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

        let mut reader = Reader::from_str(&contents);
        reader.config_mut().trim_text(true);

        //let mut host = HashMap::new();
        //let mut ports = Vec::new();
        let mut entryD= entryData::default();
        loop {
            match reader.read_event () {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        QName(b"host") => {
                            //host.clear();
                        },
                        QName(b"address") => {
                            //let mut address =String::new();
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key == QName(b"addr") {
                                        scan.ip = attr.unescape_value().unwrap().to_string().into();
                                    }
                                }
                            }
                            //host.insert("address".to_string(), address);
                        },
                        QName(b"port") => {
                            //let mut port = HashMap::new();
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key == QName(b"portid") {
                                        entryD.port=attr.unescape_value().unwrap().to_string();
                                    }
                                }
                            }
                            //ports.push(port);
                        },

                        QName(b"state") => {
                            //let mut port = HashMap::new();
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key == QName(b"state") {
                                        entryD.state=attr.unescape_value().unwrap().to_string();
                                    }
                                }
                            }
                            //ports.push(port);
                        },
                        
                        QName(b"service") => {
                            //let mut port = HashMap::new();
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key == QName(b"service") {
                                        entryD.service=attr.unescape_value().unwrap().to_string();
                                    }
                                }
                            }
                            //ports.push(port);
                        },
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        QName(b"host") => {
                            //host.insert("ports".to_string(), ports);
                            //files.push(host);
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
            
        }
        scan.entries.push(entryD);
        entries.push(scan);
    };

    // 15 17 8 13 19
    for file in entries {
        println!("{:?}", file);
    }
    
}
