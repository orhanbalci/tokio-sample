extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};

pub struct LineCodec;

impl Codec for LineCodec {
    type In = String;
    type Out = String;
    
    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if let Some(i) = buf.as_slice().iter().position(|&b| b == b'\n') {
            let line = buf.drain_to(i);
            buf.drain_to(1);
            match str::from_utf8(line.as_slice()) {
                Ok(s) => Ok(Some(s.to_string())),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid UTF8")),
            }
        }
        else{
            Ok(None)
        }
    }

    fn encode(&mut self, message : String, buf: &mut Vec<u8>) -> io::Result<()> {
        buf.extend(message.as_bytes());
        buf.push(b'\n');
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
