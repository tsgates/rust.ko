#!/usr/bin/rust run

use std::str::{from_utf8};
use std::path::Path;
use std::os;
use std::io::Command;
use std::io::File;
use std::num::from_str_radix;

// argument: name of a .ko-file containing 'rust_main'
// result:   relocation section attributes (entries, offset)
fn readelf(file: &Path) -> (uint, uint) {
    let parse = |s: &str| {
        for line in s.lines() {
            if line.starts_with("Relocation section '.rela.text.rust_main'") {
                let x1 : Vec<&str> = line.words().collect();
                println!("{}", x1);
                let ent: uint = from_str_radix(x1[7], 10).unwrap();
                let off: uint = from_str_radix(x1[5].slice_from(2), 16).unwrap();
                return (ent, off);
            }
        }
        return (0, 0);
    };
    let filename = file.as_str().unwrap();
    match Command::new("readelf").arg("-r").arg(filename).output() {
        Err(e)   => fail!("failed to execute readelf: {}", e),
        Ok (out) => from_utf8(out.output.as_slice()).map(parse).unwrap()
    }
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

    let mut buf = match File::open(filepath).read_to_end() {
        Err(e)   => fail!("failed to open output file: {}", e),
        Ok (res) => res
    };

    let (ent, off) = readelf(filepath);
    patch(ent, off, buf.as_mut_slice());

    let mut file = File::create(filepath);
    file.write(buf.as_slice()).unwrap();
}
