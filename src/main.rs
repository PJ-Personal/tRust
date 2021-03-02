mod bencode;

fn main() {
    let bencode_torrent: bencode::BencodeTorrent = bencode::get_bencode_from_file();

    match bencode_torrent.announce {
        Some(x) => println!("announce: {}", x),
        None    => println!("Err")
    }
}