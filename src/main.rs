use brc;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let path = match std::env::args().skip(1).next() {
        Some(path) => path,
        None => "./measurements_1br_symlink.txt".to_owned(),
    };
    if !std::path::Path::new(&path).is_file() {
        panic!("Missing {path}, could be that you have to create 1brc file with $ cd generate_1b_row; cargo run ");
    };
    brc::main_brc(&path, true);
    let duration = start.elapsed();
    println!("Durtation: {duration:.3?}  (Mac 3.409s)");
}
