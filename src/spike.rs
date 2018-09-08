#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;
extern crate hyper;

use hyper::{Client, Uri};
use hyper::client::{connect::Connect, HttpConnector};
use hyper_tls::HttpsConnector;

fn build_client() -> Result<Client<HttpsConnector<HttpConnector>>, hyper_tls::Error> {
    let https = HttpsConnector::new(4)?;
    Ok(Client::builder().build::<_, hyper::Body>(https))
}

pub async fn get_status<C>(client: &Client<C>, uri: Uri) -> Result<http::status::StatusCode, hyper::Error>
where C: 'static + Connect
{
    await!(client.get(uri)).map(|r| r.status())
}

pub fn main() {

    tokio::run_async(async {
        let client = build_client().unwrap();

        let uri = "https://httpbin.org/ip".parse::<Uri>().unwrap();

        let status = await!(get_status(&client, uri)).unwrap();

        println!("Status: {}", status);
    });
}
