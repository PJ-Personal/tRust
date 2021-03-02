use std::vec::Vec;
use lava_torrent::torrent::v1::Torrent;

pub type Piece = Vec<u8>;

pub struct BencodeInfo {
    pub pieces: Vec<Piece>,
    pub piece_length: i64,
    pub length: i64,
    pub name: String
}

pub struct BencodeTorrent {
    pub announce: Option<String>,
    pub info: BencodeInfo
}

pub fn get_bencode_from_file() -> BencodeTorrent {
    let url: String = String::from("/home/pj/documents/rust_torrent/Docs/debian-10.3.0-amd64-netinst.iso.torrent");

    let raw_bencode = get_raw_bencode(url);
    let request = populate_request(raw_bencode);

    return request;
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