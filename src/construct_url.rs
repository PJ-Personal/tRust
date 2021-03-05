use crate::bencode;

use bencode::{BencodeTorrent, Piece};

use url::Url;
use std::vec::Vec;
use rand::Rng;
use itertools::Itertools;

#[derive(Clone)]
struct TorrentFile {
    announce: String,
    info_hash: String,
    piece_hashes: Vec<u8>,
    piece_length: i64,
    length: i64,
    name: String,
}

const PORT_CONSTANT: u16 = 6881;

pub fn get_url(bencode_torrent: BencodeTorrent) -> String {
    let torrent_file: TorrentFile = bencode_torrent_to_torrent_file(bencode_torrent);
    let mut peer_ids = rand::thread_rng().gen::<[u8; 20]>();
    let track_url: String = build_tracker_url(&mut peer_ids, PORT_CONSTANT, torrent_file);

    return track_url;
}

fn build_tracker_url(peer_ids: &mut [u8; 20], port: u16, torrent_file: TorrentFile) -> String {
    let peer_id: Vec<u8> = peer_ids.to_vec();

    let params = &[
        ("info_hash", torrent_file.info_hash),
        ("peer_id", peer_id.iter().join("")),
        ("port", port.to_string()),
        ("uploaded", String::from("0")),
        ("downloaded", String::from("0")),
        ("compact", String::from("1")),
        ("left", torrent_file.length.to_string())
    ];

    let url = Url::parse_with_params(&torrent_file.announce, params).unwrap();

    return String::from(url.as_str());
}

fn generate_info_hash(pieces: &Piece) -> String {
    let mut vec: Vec<String> = Vec::new();

    for piece in pieces.iter() {
        let piece_to_hex = &piece;
        vec.push(format!("{:X}", piece_to_hex));
    }

    return vec.join("%");
}

fn bencode_torrent_to_torrent_file(bencode: BencodeTorrent) -> TorrentFile {
    let torrent_file = TorrentFile {
        announce: bencode.announce,
        info_hash: generate_info_hash(&bencode.info.pieces.clone()),
        piece_hashes: bencode.info.pieces,
        piece_length: bencode.info.piece_length,
        length: bencode.info.length,
        name: bencode.info.name,
    };

    return torrent_file;
}