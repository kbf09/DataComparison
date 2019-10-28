use std::fs;
use std::io::{Read};
use std::collections::HashMap;

extern crate clap;
use clap::{App, Arg};

fn main() {

    let matches = App::new("data_comparison")
        .arg(
            Arg::with_name("src_path")
                .short("s")
                .long("src_path")
                .required(true)
                .takes_value(true)
        )
        .arg(
            Arg::with_name("dst_path")
                .short("d")
                .long("dst_path")
                .required(true)
                .takes_value(true)
        ).get_matches();

    let src_path = matches.value_of("src_path").unwrap();
    let dst_path = matches.value_of("dst_path").unwrap();

    let mut src_f = fs::File::open(src_path).expect("src file not found");
    let mut dst_f = fs::File::open(dst_path).expect("dst file not found");

    let mut src_contents = String::new();
    let mut dst_contents = String::new();

    src_f.read_to_string(&mut src_contents).expect("error");
    dst_f.read_to_string(&mut dst_contents).expect("error");
    
    let src_vec: Vec<&str> = src_contents.split('\n').collect();
    let dst_vec: Vec<&str> = dst_contents.split('\n').collect();

    let mut map: HashMap<&str, u8> = HashMap::new();
    for data in dst_vec {
        map.insert(data, 0);
    }

    for data in src_vec {
        if let Some(_) = map.get(data) {
            println!("found -> {}", data);
        }
    }
}