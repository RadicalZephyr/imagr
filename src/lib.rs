#[macro_use]
extern crate tokio;

#[macro_use]
extern crate serde_derive;

use futures::{future, stream, Future, Stream};

use hyper::Client;
use hyper::client::{connect::Connect, HttpConnector};

mod photos;
pub use self::photos::Photo;

struct Blog<C> {
    client: Client<C>,
    api_key: String,
    blog_identifier: String,
}

impl<C> Blog<C> 
where C: 'static + Connect
{
    fn fetch_page_count(&self) -> impl Future<Item = usize, Error = ()> {
        future::ok(0)
    }

    fn fetch_page(&self, _page_index: usize) -> impl Stream<Item = usize, Error = ()> {
        stream::iter_ok(vec![])
    }
}
