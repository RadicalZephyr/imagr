// #![deny(warnings)]
extern crate hyper;
extern crate hyper_tls;
extern crate pretty_env_logger;

use std::env;
use std::io::{self, Write};

use hyper::rt::{self, Future, Stream};
use hyper::{client::HttpConnector, Client};
use hyper_tls::HttpsConnector;

#[derive(Debug)]
enum Error {
    InvalidArgument,
    HyperTls(hyper_tls::Error),
}

impl From<hyper_tls::Error> for Error {
    fn from(error: hyper_tls::Error) -> Self {
        Error::HyperTls(error)
    }
}

fn main() -> Result<(), Error> {
    pretty_env_logger::init();

    // Some simple CLI args requirements...
    let blog_identifier = match env::args().nth(1) {
        Some(blog_identifier) => blog_identifier,
        None => {
            println!("Usage: imagr <blog_identifier>");
            return Err(Error::InvalidArgument);
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
