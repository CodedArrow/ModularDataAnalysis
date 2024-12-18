#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod Graphing;
pub mod Nmap;
use Graphing::graph;
use Nmap::{entryData, nmapMapping};

fn main() {
    let entries=nmapMapping();
    graph(entries);
}
