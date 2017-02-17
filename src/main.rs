extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

use std::io;
use std::str;
use tokio_core::io::{Codec, EasyBuf};
use tokio_proto::pipeline::ServerProto;
use tokio_core::io::{Io, Framed};
use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use tokio_proto::TcpServer;

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

pub struct LineProto;

impl<T: Io+ 'static> ServerProto<T> for LineProto{
    type Request = String;
    type Response = String;
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;
    fn bind_transport(&self, io : T)->Self::BindTransport {
        Ok(io.framed(LineCodec))
    }    
}

pub struct Echo;

impl Service for Echo {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, request : Self::Request) -> Self::Future {
        future::ok(request).boxed() 
    }
}

fn main() {
    let address = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(LineProto, address);
    server.serve(||Ok(Echo));
    println!("Running server on 0.0.0.0:12345");
}
