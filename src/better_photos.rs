use crate::photos;

#[derive(Debug)]
pub struct Post {
    pub id: u64,
    pub slug: String,
    pub photos: Vec<Photo>,
}

#[derive(Debug)]
pub struct Photo {
    pub url: String,
}

impl From<photos::PhotoContainer> for Photo {
    fn from(photo_container: photos::PhotoContainer) -> Photo {
        Photo {
            url: photo_container.photo.url,
        }
    }
}

impl From<photos::Post> for Post {
    fn from(post: photos::Post) -> Post {
        let photos::Post { id, slug, photos } = post;
        let photos = photos
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(Photo::from)
            .collect();
        Post { id, slug, photos }
    }
}
