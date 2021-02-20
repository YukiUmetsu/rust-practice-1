use std::fs::{File, OpenOptions, remove_file};
use std::io::{Write, Read};

fn main() {
    // let mut file = File::create("src/example.txt").expect("create failed");     // if create fails error message
    // file .write_all("Hello world!\n".as_bytes()).expect("write failed");        // overwrite

//     let mut file = OpenOptions::new().append(true)
//                     .open("src/example.txt")
//                     .expect("can't open file");
//
//     file.write_all("Adding content to the file!\n".as_bytes()).expect("write failed");

//     let mut file = File::open("src/example.txt").expect("open failed");
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).expect("read failed");
//     println!("{}", contents);

    remove_file("src/example.txt").expect("remove failed");
}
