use std::fs;
use std::vec::Vec;
use lava_torrent::torrent::v1::Torrent;

pub type Piece = Vec<u8>;

struct BencodeInfo {
    pieces: Vec<Piece>,
    piece_length: i64,
    length: i64,
    name: String
}

struct BencodeTorrent {
    announce: Option<String>,
    info: BencodeInfo
}

fn main() {
    let url: String = String::from("/home/pj/documents/rust_torrent/Docs/debian-10.3.0-amd64-netinst.iso.torrent");

    let raw_bencode = get_raw_bencode(url);
    let request = populate_request(raw_bencode);

    match request.announce {
        Some(x) => println!("announce: {}", x),
        None    => println!("Err")
    }
    println!("piece_length: {}", request.info.piece_length);
    for piece in request.info.pieces.iter() {
        println!("piece {} ", piece[0]);
    }
}

fn get_raw_bencode(url: String) -> Torrent {
    
    let torrent = Torrent::read_from_file(url).unwrap();

    return torrent;
}

fn populate_request(torrent: Torrent) -> BencodeTorrent {
    let bencode_info = BencodeInfo {
        pieces: torrent.pieces,
        piece_length: torrent.piece_length,
        length: torrent.length,
        name: torrent.name
    };

    let bencode_torrent = BencodeTorrent {
        announce: torrent.announce,
        info: bencode_info
    };

    return bencode_torrent;
}