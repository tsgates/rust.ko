#!/usr/bin/rust run

use core::*;
use core::int::*;
use core::str::*;

fn hex(s: &str) -> uint {
    let digits = substr(s, 2, len(s) - 2);
    return from_str_radix(digits, 16).get() as uint;
}

fn words(s: &str) -> ~[~str] {
    let mut v: ~[~str] = ~[];
    for str::each_word(s) |tok| { v.push(~"" + tok); }
    return v;
}

fn read_bytes(pn: &str) -> ~[u8] {
    let ok = io::file_reader(~path::Path(pn));
    if !ok.is_ok() {
        println("Failed to open");
        os::set_exit_status(-1);
    }
    return ok.unwrap().read_whole_stream();
}

fn main() {
    let args = os::args();
    if args.len() != 2 {
        println("[usage] fixup [ko]");
        return;
    }
    
    let file = ~"" + args[1];
    let elf = run::program_output("readelf", [~"-r", ~"" + file]);

    let mut ent = 0u;
    let mut off = 0u;
    for str::each_line(elf.out) |line| {
        if str::starts_with(line, "Relocation section '.rela.text'") {
            off = hex(words(line)[5]);
            ent = from_str(words(line)[7]).get() as uint;
            break;
        }
    }

    let mut buf = read_bytes(file);
    let mut i = 0u;
    while i < ent {
        let rel = off + 24*i + 8;
        if buf[rel] == 0x4 {
            println(fmt!("Fixup: 0x%x", rel));
            buf[rel] = 0x2;
        }
        i += 1;
    }

    let ok = io::file_writer(~path::Path(file), [io::NoFlag]);
    if ok.is_ok() {
        let fd = ok.unwrap();
        fd.write(buf);
    }
}
