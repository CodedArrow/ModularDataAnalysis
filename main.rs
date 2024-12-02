use std::fs;
use std::io::Read;
use std::collections::HashMap;
use quick_xml::events::Event;
use quick_xml::Reader;
use quick_xml::name::QName;

fn main() {
    let mut files = Vec::new();
    // 5 8 11 4 13 0 12 4 
    println!("Enter the filenames of the Nmap XML files you want to import with format filename.xml (separated by spaces):");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let filenames: Vec<&str> = input.trim().split_whitespace().collect();

    // 17 4 0 3 / 5 8 11 4 13 0 12 4 
    for filename in filenames {
        let mut file = fs::File::open(filename).expect("Failed to open file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");

        let mut reader = Reader::from_str(&contents);
        reader.config_mut().trim_text(true);

        let mut host = HashMap::new();

        loop {
            match reader.read_event () {
                Ok(Event::Start(ref e)) => {
                    match e.name() {
                        QName(b"host") => {
                            host.clear();
                        }
                        QName(b"address") => {
                            let mut address = String::new();
                            for attr in e.attributes() {
                                if let Ok(attr) = attr {
                                    if attr.key == QName(b"addr") {
                                        address = attr.unescape_value().unwrap().to_string();
                                    }
                                }
                            }
                            host.insert("address".to_string(), address);
                        }
                        _ => {}
                    }
                }
                Ok(Event::End(ref e)) => {
                    match e.name() {
                        QName(b"host") => {
                            files.push(host.clone());
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (),
            }
        }
    }

    // 18 14 17 19 
    files.sort_by(|a, b| a["address"].cmp(&b["address"]));

    // 15 17 8 13 19
    for file in files {
        println!("{:?}", file);
    }
}
