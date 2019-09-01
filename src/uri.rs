use std::{borrow::Cow, collections::HashMap, fmt};

use surf::url::{ParseError, Url};

pub struct QueryParameters<'a>(HashMap<&'static str, Cow<'a, str>>);

impl<'a> QueryParameters<'a> {
    pub fn new(inner: HashMap<&'static str, Cow<'a, str>>) -> QueryParameters<'a> {
        QueryParameters(inner)
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

pub fn tumblr_uri(
    blog_identifier: impl AsRef<str>,
    path: &UriPath,
    query_params: &QueryParameters,
) -> String {
    format!(
        "https://api.tumblr.com/v2/blog/{blog_identifier}/{path}?{query_params}",
        blog_identifier = blog_identifier.as_ref(),
        path = path,
        query_params = query_params,
    )
}
