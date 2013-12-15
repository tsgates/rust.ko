#!/usr/bin/rust run

use std::str::{StrSlice, from_utf8};
use std::from_str::from_str;
use std::io;
use std::path::Path;
use std::os;
use std::run;
use std::to_str::ToStr;
use std::num::from_str_radix;

fn hex(s: &str) -> uint {
    let digits = s.slice_from(2);
    return from_str_radix(digits, 16).unwrap();
}

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println("[usage] fixup [ko]");
        return;
    }
    
    let file = ~"" + args[1];
    let elf = run::process_output("readelf", [~"-r", ~"" + file]);

    let mut ent = 0u;
    let mut off = 0u;
    let out_str = from_utf8(elf.output);
    for line in out_str.lines() {
        if line.starts_with("Relocation section '.rela.text'") {
            let x1 : ~[&str] = line.words().collect();
            off = hex(x1[5]);
            ent = from_str::<uint>(x1[7]).unwrap();
            break;
        }
    }
    let mut buf = io::File::open(&Path::new(file.clone())).read_to_end();
    let mut i = 0u;
    while i < ent {
        let rel = off + 24*i + 8;
        if buf[rel] == 0x4 {
            // TODO HEX!
            println(format!("Fixup: 0x{}", rel));
            buf[rel] = 0x2;
        }
        i += 1;
    }

    let mut file = io::File::create(&Path::new(file));
    file.write(buf);
}
