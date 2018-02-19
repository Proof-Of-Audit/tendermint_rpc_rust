#![allow(warnings)] // remove when error_chain is fixed

extern crate futures;
extern crate reqwest;
extern crate tokio_core;
#[macro_use]
extern crate error_chain;
extern crate tendermint_rpc;

use std::mem;
use std::io::{self, Cursor,Read};
use futures::{Future, Stream};
use reqwest::unstable::async::{Client, Decoder};
use tendermint_rpc::response_to_result;

error_chain! {
    foreign_links {
        ReqError(reqwest::Error);
        IoError(io::Error);
    }
}

fn run() -> Result<()> {
    let mut core = tokio_core::reactor::Core::new()?;
    let client = Client::new(&core.handle());

    let work = client.get("http://35.204.86.158:46657/status")
        .send()
        .map_err(|e| Error::from(e))
        .and_then(|mut res| {
            let body = mem::replace(res.body_mut(), Decoder::empty());
            body.concat2().map_err(Into::into)
        })
        .and_then(|body| {
            let mut body = Cursor::new(body);
            let mut data = String::new();
            body.read_to_string(&mut data);
            let result = response_to_result(&data).unwrap();
            println!("Latest Block Height:{}", result.c);         
            Ok(result)
        });

    core.run(work)?;
    Ok(())
}

quick_main!(run);