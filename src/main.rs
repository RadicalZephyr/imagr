use std::{env, process};

use failure::Error;
use failure_derive::Fail;

use futures::future::{join_all, FutureExt, TryFutureExt};

use hyper::{self, client::HttpConnector, Client};
use hyper_tls::{self, HttpsConnector};

use pretty_env_logger;

use imagr::Blog;

#[derive(Debug, Fail)]
#[fail(display = "invalid argument")]
struct InvalidArgument;

async fn download_blog_photos(api_key: String, blog_identifier: String) -> Result<(), Error> {
    let client = surf::Client::new();
    let blog = Blog::new(client, api_key, blog_identifier);

    let post_count = blog.fetch_post_count().await.unwrap();
    let mut page_start_index = 0;

    while page_start_index < post_count {
        let photos = blog.fetch_posts_page(page_start_index).await.unwrap();
        let received = photos.len();
        let files = photos.into_iter().map(|post| blog.download_post(post));
        join_all(files).await;
        page_start_index += received;
    }
    Ok(())
}

async fn run() -> Result<(), Error> {
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

    // TODO: Make it return a Result.
    download_blog_photos(api_key, blog_identifier).await?;

    Ok(())
}

fn main() {
    match runtime::raw::enter(runtime::native::Native, async { run().await }) {
        Ok(()) => {}
        Err(err) => {
            println!("Usage: imagr <blog_identifier>\nError: {}", err);
            process::exit(1);
        }
    }
}
