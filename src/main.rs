// #![deny(warnings)]
extern crate failure;
extern crate hyper;
extern crate hyper_tls;
extern crate pretty_env_logger;

#[macro_use]
extern crate failure_derive;

use std::env;
use std::io::{self, Write};
use std::process;

use failure::Error;

use hyper::rt::{self, Future, Stream};
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;

#[derive(Debug, Fail)]
#[fail(display = "invalid argument")]
struct InvalidArgument;

fn main() {
    match run() {
        Ok(()) => {}
        Err(err) => {
            println!("Usage: imagr <blog_identifier>\nError: {}", err);
            process::exit(1);
        }
    }
}

fn run() -> Result<(), Error> {
    pretty_env_logger::init();

    // Some simple CLI args requirements...
    let blog_identifier = match env::args().nth(1) {
        Some(blog_identifier) => blog_identifier,
        None => {
            return Err(InvalidArgument.into());
        }
    };

    let client = gimme_a_client()?;

    // Run the runtime with the future trying to fetch and print this URL.
    //
    // Note that in more complicated use cases, the runtime should probably
    // run on its own, and futures should just be spawned into it.
    // rt::run(fetch_url(url));
    Ok(())
}

fn gimme_a_client() -> Result<Client<HttpsConnector<HttpConnector>>, hyper_tls::Error> {
    let https = HttpsConnector::new(4)?;
    Ok(Client::builder().build::<_, hyper::Body>(https))
}
