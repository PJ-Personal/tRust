extern crate rustc_serialize;

use std::fs::File;
use std::path::Path;
use std::io::Read;
//use bip_bencode::{BencodeRef, BRefAccess, BDecodeOpt};


#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct BencodeInfo {
    pieces: String,
    piece_length: u64,
    length: u64,
    name: String,
}

#[derive(RustcEncodable, RustcDecodable, PartialEq)]
struct BencodeTorrent {
    announce: String,
    info: BencodeInfo,
}

fn read_file(file_name: String) -> std::io::Result<Vec<u8>> {
    let mut file = File::open(file_name)?;

    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    return Ok(data);
}

fn main() {
    let path = String::from("/home/pj/documents/rust_torrent/Docs/ubuntu-20.04.2.0-desktop-amd64.iso.torrent");

    match read_file(path) {
        Ok(mut file) => println!("{}", String::from_utf8_lossy(&mut file)),
        Err(e) => eprintln!("broke: {}", e),
    }
}