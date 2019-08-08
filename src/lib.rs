#![feature(async_await)]

extern crate tokio;

#[macro_use]
extern crate serde_derive;

use futures::{future, stream, Future, Stream};

use hyper::client::{connect::Connect, HttpConnector};
use hyper::Client;

mod photos;
pub use self::photos::Photo;

mod macros;

mod uri;

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
        let path = uri_path![posts/photos];
        let params = uri_params!{ api_key => &self.api_key, limit => "1" };
        let uri = uri::tumblr_uri(&self.blog_identifier, &path, &params);
        0
    }
}
