#![feature(slice_patterns)]

mod utils;

use cfg_if::cfg_if;
use js_sys;
use serde_derive::Serialize;
use std::io::{Cursor, Read};
use tar::parse_tar;
use wasm_bindgen::prelude::*;
use zip;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        use wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

// Uncomment for debugging
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, Clone)]
enum Archive {
    Zip(Vec<u8>),
    Tar(Vec<u8>),
    Unknown(Vec<u8>),
}

impl From<Vec<u8>> for Archive {
    fn from(buffer: Vec<u8>) -> Archive {
        match buffer.as_slice() {
            [0x50, 0x4B, 0x03, 0x04, ..]
            | [0x50, 0x4B, 0x05, 0x06, ..]
            | [0x50, 0x4B, 0x07, 0x08, ..] => Archive::Zip(buffer),
            _ if buffer.len() > 0x101
                && match buffer.split_at(0x101).1 {
                    [0x75, 0x73, 0x74, 0x61, 0x72, 0x00, 0x30, 0x30, ..]
                    | [0x75, 0x73, 0x74, 0x61, 0x72, 0x20, 0x20, 0x00, ..] => true,
                    _ => false,
                } =>
            {
                Archive::Tar(buffer)
            }
            _ => Archive::Unknown(buffer),
        }
    }
}

#[derive(Serialize, Debug, Clone, Default)]
struct File {
    pub path: String,
    pub buffer: Vec<u8>,
}

#[wasm_bindgen]
pub fn unarchive(buffer: js_sys::Uint8Array) -> Result<js_sys::Array, wasm_bindgen::JsValue> {
    let mut archive_bytes = vec![0; buffer.byte_length() as usize];
    buffer.copy_to(&mut archive_bytes);

    let files = js_sys::Array::new();

    match Archive::from(archive_bytes) {
        Archive::Zip(buffer) => {
            let mut archive = zip::ZipArchive::new(Cursor::new(&buffer)).unwrap();
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).unwrap();

                let path = String::from(file.sanitized_name().to_string_lossy());
                let mut buffer = vec![];
                file.read_to_end(&mut buffer).unwrap();

                let file = File { path, buffer };
                files.push(&wasm_bindgen::JsValue::from_serde(&file).unwrap());
            }
        }
        Archive::Tar(buffer) => {
            let (_, entries) = parse_tar(&buffer).unwrap();
            for entry in entries.iter() {
                let path = String::from(entry.header.name);
                let buffer = Vec::from(entry.contents);

                let file = File { path, buffer };
                files.push(&wasm_bindgen::JsValue::from_serde(&file).unwrap());
            }
        }
        Archive::Unknown(buffer) => {
            return Err(wasm_bindgen::JsValue::from_str(&format!(
                "Unknown file type for buffer:\n{:?}",
                buffer
            )));
        }
    };

    Ok(files)
}
