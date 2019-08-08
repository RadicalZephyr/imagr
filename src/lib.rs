#![feature(async_await)]

extern crate tokio;

#[macro_use]
extern crate serde_derive;

use futures::{future, stream, Future, Stream};

use hyper::client::{connect::Connect, HttpConnector};
use hyper::Client;

mod photos;
pub use self::photos::Photo;

mod uri;
use crate::uri::{QueryParameters};

pub struct Blog<C> {
    client: Client<C>,
    api_key: String,
    blog_identifier: String,
}

impl<C> Blog<C>
where
    C: 'static + Connect,
{
    pub fn new(client: Client<C>, api_key: String, blog_identifier: String) -> Blog<C> {
        Blog { client, api_key, blog_identifier }
    }

    async fn fetch_page_count(&self) -> usize {
        0
    }
}
