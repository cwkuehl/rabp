use std::{io::Write, fs};

fn main() {
    // Save timestamp.
    let outdir = std::env::var("OUT_DIR").unwrap();
    let outfile = format!("{}/timestamp.txt", outdir);

    let mut fh = fs::File::create(&outfile).unwrap();
    write!(fh, r#""{}""#, chrono::Local::now()).ok();
}
