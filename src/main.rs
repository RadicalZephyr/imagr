#![feature(async_await)]

extern crate tokio;

use hyper;
use hyper_tls;
use pretty_env_logger;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;

use std::{env, fmt, process};
use std::collections::HashMap;
use std::io::Write;

use failure::Error;
use futures::{compat::Future01CompatExt, future::{FutureExt, TryFutureExt},};

use http::uri::{self, Uri};
use hyper::client::{connect::Connect, HttpConnector};
use hyper::Client;
use hyper_tls::HttpsConnector;

#[derive(Debug, Fail)]
#[fail(display = "invalid argument")]
struct InvalidArgument;

struct QueryParameters<'a>(HashMap<&'a str, &'a str>);
struct UriPath(Vec<&'static str>);

impl<'a> QueryParameters<'a> {
    fn with_api_key(api_key: &'a str) -> QueryParameters<'a> {
        let mut query_params = HashMap::new();
        query_params.insert("api_key", api_key);
        QueryParameters(query_params)
    }
}

impl<'a> fmt::Display for QueryParameters<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<String>>()
                .join("&")
        )
    }
}

impl fmt::Display for UriPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("/"))
    }
}

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

    // TODO: Custom missing env var error message.
    let api_key = env::var("IMAGR_TOKEN")?;

    let futures_03_future = async {
        let client = build_client().unwrap();

        let uri = photo_posts_uri(blog_identifier, api_key).unwrap();

    };

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

fn photo_posts_uri(
    blog_identifier: impl AsRef<str>,
    api_key: impl AsRef<str>,
) -> Result<Uri, uri::InvalidUri> {
    let path = UriPath(vec!["posts", "photo"]);
    let query_params = QueryParameters::with_api_key(api_key.as_ref());
    tumblr_uri(blog_identifier, &path, &query_params)
}

fn tumblr_uri(
    blog_identifier: impl AsRef<str>,
    path: &UriPath,
    query_params: &QueryParameters,
) -> Result<Uri, uri::InvalidUri> {
    format!(
        "https://api.tumblr.com/v2/blog/{blog_identifier}/{path}?{query_params}",
        blog_identifier = blog_identifier.as_ref(),
        path = path,
        query_params = query_params,
    ).parse()
}
