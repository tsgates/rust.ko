extern crate core;

use core::fmt::Display;
use std::fs::File;
use std::os;
use std::path::Path;
use std::process::Command;
use std::io::Read;
use std::io::Write;
use std::num::from_str_radix;
use std::str::{from_utf8};

// argument: name of a .ko-file containing 'rust_main'
// result:   relocation section attributes (entries, offset)
fn readelf(file: &Path) -> (uint, uint) {
    let filename = file.to_str().unwrap();
    match Command::new("readelf").arg("-r").arg(filename).output() {
        Err(e)   => panic!("failed to execute readelf: {}", e),
        Ok (out) => from_utf8(out.stdout.as_slice()).map(parse).unwrap()
    }
}

fn print_vec<T: Display>(v: &[T]) {
    for i in v.iter() {
        println!("{}", i)
    }
}

fn parse(s: &str) -> (uint, uint) {
    for line in s.lines() {
        if line.starts_with("Relocation section '.rela.text.rust_main'") {
            let x1 : Vec<&str> = line.words().collect();
            print_vec(x1.as_slice());
            let ent: uint = from_str_radix(x1[7], 10).unwrap();
            let off: uint = from_str_radix(x1[5].slice_from(2), 16).unwrap();
            return (ent, off);
        }
    }
    return (0, 0);
}

fn patch(ent: uint, off: uint, buf: &mut [u8]) {
    for i in range (0, ent) {
        let rel = off + 24*i + 8;
        if buf[rel] == 0x4 {
            println!("Fixup: 0x{}", rel);
            buf[rel] = 0x2;
        }
    }
}

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("[usage] fixup [ko]");
        return;
    }
    let filepath = &Path::new(args[1].as_slice());

    let mut buf: Vec<u8> = Vec::new();
    if File::open(filepath).unwrap().read_to_end(&mut buf).is_err() {
        panic!("failed to open or read file: {}", filepath.to_str().unwrap());
    }

    let (ent, off) = readelf(filepath);
    patch(ent, off, buf.as_mut_slice());

    let mut file = File::create(filepath).unwrap();
    file.write(buf.as_slice()).unwrap();
}
