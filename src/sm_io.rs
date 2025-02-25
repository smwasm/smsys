use json::object;
use std::fs::File;
use std::io::prelude::*;

use smcore::{smh, smu};
use smdton::{SmDton, SmDtonBuffer, SmDtonMap};

fn _sm_read_file(_input: &SmDtonBuffer) -> SmDtonBuffer {
    let sd = SmDton::new_from_buffer(_input);
    let mut smap = SmDtonMap::new();

    let _op = sd.get_string("path");
    match _op {
        Some(_path) => {
            let mut _rst = File::open(&_path);
            match _rst {
                Ok(mut _f) => {
                    let mut _buf: Vec<u8> = Vec::new();
                    let _a = _f.read_to_end(&mut _buf);
                    let _txt = String::from_utf8(_buf).unwrap();
                    smap.add_string("path", &_path);
                    smap.add_string("data", &_txt);
                    return smap.build();
                }
                _ => {}
            }
        }
        _ => {}
    }
    return smap.build();
}

fn _sm_get_ms(_input: &SmDtonBuffer) -> SmDtonBuffer {
    let mut smap = SmDtonMap::new();

    let ms = smu.get_current_ms() as i64;
    smap.add_i64("ms", ms);
    smap.add_string("from", "---[rust]---");

    return smap.build();
}

pub fn _sm_init() {
    smu.log(&format!("--- sm init --- {} --- {} ---", "sys", "SmIo"));

    let mut _define1 = object! {
        "$usage" => "sys.io.read.file",
        "format" => "utf8",
        "path" => "",
    };
    smh.register_by_json(&_define1, _sm_read_file);

    let mut _define2 = object! {
        "$usage" => "sys.get.ms",
    };
    smh.register_by_json(&_define2, _sm_get_ms);
}
