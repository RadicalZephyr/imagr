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

use failure::Error;
use futures::future::{join_all, FutureExt, TryFutureExt};

use hyper::client::HttpConnector;
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

async fn download_blog_photos(api_key: String, blog_identifier: String) {
    let client = build_client().unwrap();
    let blog = Blog::new(client, api_key, blog_identifier);

    let post_count = blog.fetch_post_count().await.unwrap();
    let mut page_start_index = 0;

    while page_start_index < post_count {
        let photos = blog.fetch_posts_page(page_start_index).await.unwrap();
        let received = photos.len();
        let files = photos.into_iter().map(|post| blog.download_file(post));
        join_all(files).await;
        page_start_index += received;
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

    let futures_03_future = download_blog_photos(api_key, blog_identifier);
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
