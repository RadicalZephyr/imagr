use std::collections::HashMap;
use std::fmt;

use http::uri::{self, Uri};

pub struct QueryParameters<'a>(HashMap<&'a str, &'a str>);

impl<'a> QueryParameters<'a> {
    fn with_api_key(api_key: &'a str) -> QueryParameters<'a> {
        let mut query_params = HashMap::new();
        query_params.insert("api_key", api_key);
        QueryParameters(query_params)
    }
}

impl<'a> fmt::Display for QueryParameters<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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

pub struct UriPath(Vec<&'static str>);

impl UriPath {
    pub fn new(inner: Vec<&'static str>) -> UriPath {
        UriPath(inner)
    }
}

impl fmt::Display for UriPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.join("/"))
    }
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
    )
    .parse()
}
