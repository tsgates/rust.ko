#!/usr/bin/rust run

use std::str::{StrSlice, from_utf8};
use std::from_str::from_str;
use std::io;
use std::path::Path;
use std::os;
use std::io::Command;
use std::num::from_str_radix;

fn hex(s: &str) -> uint {
    let digits = s.slice_from(2);
    return from_str_radix(digits, 16).unwrap();
}

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println!("[usage] fixup [ko]");
        return;
    }
    
    let file = args.get(1).to_string();
    let elf_out = Command::new("readelf").arg("-r").arg(file.clone()).output();

    let mut ent = 0u;
    let mut off = 0u;
    if elf_out.is_err() {
      fail!("failed to execute readelf: {}", elf_out.err());
    }
    let out = elf_out.ok().unwrap().output;
    let out_str = from_utf8(out.as_slice()).unwrap();
    for line in out_str.lines() {
        if line.starts_with("Relocation section '.rela.text.rust_main'") {
            let x1 : Vec<&str> = line.words().collect();
            println!("{}", x1);
            off = hex(*x1.get(5));
            ent = from_str::<uint>(*x1.get(7)).unwrap();
            break;
        }
    }
    let result = io::File::open(&Path::new(file.clone())).read_to_end();
    if result.is_err() {
      fail!("failed to open output file: {}", result.err());
    }
    let mut buf_vec = result.ok().unwrap();
    let mut buf = buf_vec.as_mut_slice();
    let mut i = 0u;
    while i < ent {
        let rel = off + 24*i + 8;
        if buf[rel] == 0x4 {
            println!("Fixup: 0x{}", rel);
            buf[rel] = 0x2;
        }
        i += 1;
    }

    let mut file = io::File::create(&Path::new(file));
    file.write(buf.as_slice());
}
