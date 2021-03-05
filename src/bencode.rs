extern crate bip_bencode;

use std::vec::Vec;
use std::fs;
use bip_bencode::{BencodeRef, BRefAccess, BDecodeOpt};

pub type Piece = Vec<u8>;

#[derive(PartialEq, Eq, Debug)]
pub struct BencodeInfo {
    pub pieces: Vec<u8>,
    pub piece_length: i64,
    pub length: i64,
    pub name: String
}

#[derive(PartialEq, Eq, Debug)]
pub struct BencodeTorrent {
    pub announce: String,
    pub info: BencodeInfo
}

pub fn get_bencode_from_file() -> BencodeTorrent {
    let url: String = String::from("/home/pj/documents/rust_torrent/Docs/ILSVRC2012_img_val.tar-5d6d0df7ed81efd49ca99ea4737e0ae5e3a5f2e5.torrent");

    let file = fs::read(url).expect("unable to open file");
    let bencode_decoded = BencodeRef::decode(&file, BDecodeOpt::default());
    let bencode = match bencode_decoded {
        Ok(v) => v,
        Err(e) => panic!(e)
    };

    return populate_request(bencode);
}

fn populate_request(bencode: BencodeRef) -> BencodeTorrent {
    let info = bencode.dict().unwrap().lookup("info".as_bytes()).unwrap();

    let bencode_info = BencodeInfo {
        // I cannot express in words how much I hate this
        pieces: info.dict().unwrap().lookup("pieces".as_bytes()).unwrap().bytes().unwrap().to_vec(),
        piece_length: info.dict().unwrap().lookup("piece length".as_bytes()).unwrap().int().unwrap(),
        length: info.dict().unwrap().lookup("length".as_bytes()).unwrap().int().unwrap(),
        name: String::from(info.dict().unwrap().lookup("name".as_bytes()).unwrap().str().unwrap()),
    };

    let bencode_torrent = BencodeTorrent {
        announce: String::from(bencode.dict().unwrap().lookup("announce".as_bytes()).unwrap().str().unwrap()),
        info: bencode_info
    };

    return bencode_torrent;
}