extern crate fatfs;
extern crate chrono;

use std::env;
use std::fs::File;
use chrono::{DateTime, Local};

use fatfs::{FileSystem, FsOptions, BufStream};

fn format_file_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024*KB;
    const GB: u64 = 1024*MB;
    if size < 1024 {
        format!("{}B", size)
    } else if size < MB {
        format!("{}KB", size / KB)
    } else if size < GB {
        format!("{}KB", size / MB)
    } else {
        format!("{}KB", size / GB)
    }
}

fn main() {
    let file = File::open("resources/fat32.img").unwrap();
    let mut buf_rdr = BufStream::new(file);
    let fs = FileSystem::new(&mut buf_rdr, FsOptions::new()).unwrap();
    let mut root_dir = fs.root_dir();
    let dir = match env::args().nth(1) {
        None => root_dir,
        Some(ref path) if path == "." => root_dir,
        Some(ref path) => root_dir.open_dir(&path).unwrap(),
    };
    for r in dir.iter() {
        let e = r.unwrap();
        let modified = DateTime::<Local>::from(e.modified()).format("%Y-%m-%d %H:%M:%S").to_string();
        println!("{:4}  {}  {}", format_file_size(e.len()), modified, e.file_name());
    }
}
