mod bencode;
mod construct_url;

extern crate crypto;

use bencode::{BencodeTorrent, get_bencode_from_file};
use construct_url::{get_url};
use std::fs::File;
use std::io::{Write};

fn main() {
    let bencode_torrent: BencodeTorrent = get_bencode_from_file();
    let bencode_torrent_url: &mut String = &mut get_url(bencode_torrent);
    let mut file = File::create("out.txt").expect("unable to open");
    return write!(file, "{}", bencode_torrent_url).unwrap();
}
