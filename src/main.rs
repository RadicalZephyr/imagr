#![feature(async_await)]

extern crate tokio;

use hyper;
use hyper_tls;
use pretty_env_logger;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;

use std::{env, process};
use std::io::Write;

use failure::Error;
use futures::{compat::Future01CompatExt, future::{FutureExt, TryFutureExt},};

use hyper::client::{connect::Connect, HttpConnector};
use hyper::Client;
use hyper_tls::HttpsConnector;

use imagr::Blog;


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

async fn download_blog_photos(blog_identifier: String, api_key: String) {
    let client = build_client().unwrap();
    let mut blog = Blog::new(client, blog_identifier, api_key);
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

    // TODO: Custom missing env var error message.
    let api_key = env::var("IMAGR_TOKEN")?;

    let futures_03_future = download_blog_photos(blog_identifier, api_key);
    let futures_01_future = futures_03_future.unit_error().boxed().compat();
    hyper::rt::run(futures_01_future);

    Ok(())
}

fn handle_connection_error(err: hyper::Error) {
    println!("Error: {}", err);
    process::exit(1)
}

fn build_client() -> Result<Client<HttpsConnector<HttpConnector>>, hyper_tls::Error> {
    let https = HttpsConnector::new(4)?;
    Ok(Client::builder().build::<_, hyper::Body>(https))
}
