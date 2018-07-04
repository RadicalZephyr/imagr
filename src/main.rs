// #![deny(warnings)]
extern crate failure;
extern crate futures;
extern crate http;
extern crate hyper;
extern crate hyper_tls;
extern crate pretty_env_logger;

#[macro_use]
extern crate failure_derive;

use std::collections::HashMap;
use std::env;
use std::fmt;
use std::io::{self, Write};
use std::process;

use failure::Error;
use futures::{Future, Stream};

use http::uri::{self, Uri};
use hyper::client::{connect::Connect, HttpConnector};
use hyper::rt;
use hyper::Client;
use hyper_tls::HttpsConnector;

#[derive(Debug, Fail)]
#[fail(display = "invalid argument")]
struct InvalidArgument;

struct QueryParameters(HashMap<String, String>);
struct UriPath(Vec<&'static str>);

impl QueryParameters {
    fn new(api_key: impl Into<String>) -> QueryParameters {
        let mut query_params = HashMap::new();
        query_params.insert("api_key".into(), api_key.into());
        QueryParameters(query_params)
    }
}

impl fmt::Display for QueryParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

    let client = build_client()?;

    let future = tumbl(client, api_key, blog_identifier)?;
    rt::run(future.map_err(handle_connection_error));

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
    api_key: String,
) -> Result<Uri, uri::InvalidUri> {
    let path = UriPath(vec!["posts", "photo"]);
    let query_params = QueryParameters::new(api_key);
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

fn tumbl<C>(
    client: Client<C>,
    api_key: String,
    blog_identifier: String,
) -> Result<impl Future<Item = (), Error = hyper::Error>, uri::InvalidUri>
where
    C: 'static + Connect,
{
    let uri = photo_posts_uri(blog_identifier, api_key)?;
    println!("{}", uri);
    Ok(client.get(uri).and_then(|response| {
        println!("Response: {}", response.status());
        println!("Headers: {:#?}", response.headers());

        response.into_body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map_err(|_e| panic!("ahhhhhhh!"))
        })
    }))
}
