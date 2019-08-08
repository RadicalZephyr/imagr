#![feature(async_await)]

extern crate tokio;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;

use futures::compat::Future01CompatExt;

use hyper::rt::{Future, Stream};
use hyper::client::{connect::Connect, HttpConnector};
use hyper::Client;

use serde_json::{from_str, Value};

mod photos;
pub use crate::photos::Photo;

mod macros;

mod uri;

mod response;
use crate::response::{Response, Meta, TotalPosts};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "uri error: {}", _0)]
    Uri(http::uri::InvalidUri),

    #[fail(display = "hyper error: {}", _0)]
    Http(hyper::error::Error),

    #[fail(display = "json error: {}", _0)]
    Json(serde_json::error::Error),

    #[fail(display = "api error: {}", _0)]
    Api(String),
}

impl From<http::uri::InvalidUri> for Error {
    fn from(error: http::uri::InvalidUri) -> Error {
        Error::Uri(error)
    }
}

impl From<hyper::error::Error> for Error {
    fn from(error: hyper::error::Error) -> Error {
        Error::Http(error)
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Error {
        Error::Json(error)
    }
}

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

    pub async fn fetch_post_count(&self) -> Result<usize, Error> {
        let path = uri_path![posts/photo];
        let params = uri_params!{ api_key => &self.api_key, limit => "1" };
        let uri = uri::tumblr_uri(&self.blog_identifier, &path, &params)?;

        let response = self.client.get(uri).compat().await?;
        let body = response.into_body().map(hyper::Chunk::into_bytes).concat2().compat().await?;
        let v: Response<TotalPosts> = serde_json::from_slice(&body)?;

        if v.meta.is_success() {
            Ok(v.response.amount)
        } else {
            Err(Error::Api(v.meta.msg.clone()))
        }
    }
}
