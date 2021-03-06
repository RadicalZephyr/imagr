use std::cmp;

use serde_derive::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Posts {
    pub posts: Vec<Post>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Post {
    pub id: u64,
    pub slug: String,
    pub photos: Option<Vec<PhotoContainer>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct PhotoContainer {
    #[serde(rename = "original_size")]
    pub photo: Photo,
    alt_sizes: Vec<Photo>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct Photo {
    pub url: String,
    width: usize,
    height: usize,
}

impl Photo {
    fn area(&self) -> usize {
        self.width * self.height
    }
}

impl Ord for Photo {
    fn cmp(&self, other: &Photo) -> cmp::Ordering {
        self.area().cmp(&other.area())
    }
}

impl PartialOrd for Photo {
    fn partial_cmp(&self, other: &Photo) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TotalPosts {
    #[serde(rename = "total_posts")]
    pub amount: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::response::Response;

    use serde_json;

    #[test]
    fn test_json_parse() {
        let photo_size: Photo = serde_json::de::from_str("{\"width\": 1280, \"height\": 722, \"url\": \"http:\\/\\/derekg.org\\/photo\\/1280\\/7431599279\\/1\\/ tumblr_lo36wbWqqq1qanqww\"}").unwrap();
        assert_eq!(photo_size.width, 1280);
        assert_eq!(photo_size.height, 722);
    }

    #[test]
    fn test_full_parse() {
        let _response: Response<Posts> =
            serde_json::de::from_str(include_str!("response.json")).unwrap();
    }

    #[test]
    fn test_photo_size_compares_by_area() {
        let really_tall = Photo {
            width: 10,
            height: 10000,
            url: String::from(""),
        };
        let square = Photo {
            width: 100,
            height: 100,
            url: String::from(""),
        };
        assert!(really_tall > square);
    }
}
