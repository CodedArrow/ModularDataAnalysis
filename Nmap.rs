#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::fs;
use std::io::Read;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::name::QName;



#[derive(Default, Debug)]
struct entry {
    entries: Vec<entryData>,
    ip: String,
}
#[derive(Default, Debug)]
pub struct entryData {
    pub port: String,
    pub state: String,
    pub service: String,
}

pub fn nmapMapping (){
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

        let mut entryD= entryData::default();
        loop {
            match reader.read_event () {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        QName(b"host") => {
                        },
                        QName(b"address") => {
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key == QName(b"addr") {
                                        scan.ip = attr.unescape_value().unwrap().to_string().into();
                                    }
                                }
                            }
                        },
                        QName(b"port") => {
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key == QName(b"portid") {
                                        entryD.port=attr.unescape_value().unwrap().to_string();
                                    }
                                }
                            }
                        },

                        QName(b"state") => {
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key == QName(b"state") {
                                        entryD.state=attr.unescape_value().unwrap().to_string();
                                    }
                                }
                            }
                        },
                        
                        QName(b"service") => {
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key == QName(b"service") {
                                        entryD.service=attr.unescape_value().unwrap().to_string();
                                    }
                                }
                            }
                        },
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        QName(b"host") => {
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