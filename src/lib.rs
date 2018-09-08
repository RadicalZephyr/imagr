extern crate futures;
extern crate hyper;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use futures::{future, stream, Future, Stream};

use hyper::Client;
use hyper::client::{connect::Connect, HttpConnector};

mod photos;
pub use photos::Photo;

struct Blog<C> {
    client: Client<C>,
    api_key: String,
    blog_identifier: String,
}

fn fetch_page_count<C>(blog: &Blog<C>) -> impl Future<Item = usize, Error = ()>
where C: 'static + Connect,
{
    future::ok(0)
}

fn fetch_page<C>(blog: &Blog<C>, page_index: usize) -> impl Stream<Item = usize, Error = ()>
where C: 'static + Connect,
{
    stream::iter_ok(vec![])
}
