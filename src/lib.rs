#![feature(async_await)]

extern crate tokio;

#[macro_use]
extern crate failure_derive;

#[macro_use]
extern crate serde_derive;

use std::fmt;
use std::io::Cursor;

use futures::compat::Future01CompatExt;

use hyper::rt::Stream;
use hyper::client::connect::Connect;
use hyper::Client;

use serde::de::DeserializeOwned;

mod photos;
pub use crate::photos::{Posts, Post, Photo};

mod macros;

mod uri;
use crate::uri::{UriPath, QueryParameters};

mod response;
use crate::response::{Response, TotalPosts};

const MAX_PAGE_SIZE: &'static str = "20";

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

    async fn tumblr_get<'a, 'de, T>(&self, path: UriPath, params: QueryParameters<'a>) -> Result<Response<T>, Error>
    where
        T: 'static + Clone + fmt::Debug + DeserializeOwned,
    {
        let uri = uri::tumblr_uri(&self.blog_identifier, &path, &params)?;

        let response = self.client.get(uri).compat().await?;
        let body = response.into_body().map(hyper::Chunk::into_bytes).concat2().compat().await?;
        let cursor = Cursor::new(body);
        let v: Response<T> = serde_json::from_reader(cursor)?;

        if v.meta.is_success() {
            Ok(v)
        } else {
            Err(Error::Api(v.meta.msg.clone()))
        }
    }

    pub async fn fetch_post_count(&self) -> Result<usize, Error> {
        let path = uri_path![posts/photo];
        let params = uri_params!{ api_key => &self.api_key, limit => "1" };

        let v: Response<TotalPosts> = self.tumblr_get::<TotalPosts>(path, params).await?;
        Ok(v.response.amount)
    }

    pub async fn fetch_posts_page(&self, page_start_index: usize) -> Result<Vec<Post>, Error> {
        let path = uri_path![posts/photo];
        let params = uri_params!{
            api_key => &self.api_key,
            limit => MAX_PAGE_SIZE,
            offset => format!("{}", page_start_index)
        };

        let v: Response<Posts> = self.tumblr_get::<Posts>(path, params).await?;
        Ok(v.response.posts)
    }

    pub async fn download_file(&self, post: Post) {
        dbg!(post);
        panic!();
    }
}
