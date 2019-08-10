use std::fmt;

use serde::de::DeserializeOwned;

#[derive(Clone, Debug, Deserialize)]
#[serde(bound = "T: Clone + fmt::Debug + DeserializeOwned")]
pub struct Response<T>
where
    T: Clone + fmt::Debug + DeserializeOwned,
{
    pub meta: Meta,
    pub response: T,
}

// struct ResponseVisitor<T>(PhantomData<T>);

// impl<'de, T> Visitor<'de> for ResponseVisitor<T>
// where
//     T: Clone + fmt::Debug + Deserialize<'de>,
// {
//     type Value = Response<T>;

//     fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "help!")
//     }

//     fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
//     where
//         A: de::MapAccess<'de>,
//     {
//         let mut meta: Option<Meta> = None;
//         let mut response: Option<T> = None;

//         while let Some(key) = access.next_key()? {
//             match key {
//                 "meta" => {
//                     if meta.is_some() {
//                         return Err(de::Error::duplicate_field("meta"));
//                     }
//                     meta = Some(access.next_value()?);
//                 }
//                 "response" => {
//                     if response.is_some() {
//                         return Err(de::Error::duplicate_field("response"));
//                     }
//                     response = Some(access.next_value()?);
//                 }
//                 _ => (),
//             }
//         }

//         match (meta, response) {
//             (None, None) => Err(de::Error::missing_field("meta and response")),
//             (Some(_), None) => Err(de::Error::missing_field("meta")),
//             (None, Some(_)) => Err(de::Error::missing_field("response")),
//             (Some(meta), Some(response)) => Ok(Response {
//                 meta,
//                 response,
//                 _phantom: PhantomData,
//             }),
//         }
//     }
// }

// impl<'de, T> Deserialize<'de> for Response<T>
// where
//     T: 'de + Clone + fmt::Debug + Deserialize<'de>,
// {
//     fn deserialize<D>(de: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         de.deserialize_map(ResponseVisitor(PhantomData))
//     }
// }

#[derive(Clone, Debug, Deserialize)]
pub struct Meta {
    pub status: u16,
    pub msg: String,
}

impl Meta {
    pub fn is_success(&self) -> bool {
        self.status == 200
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct TotalPosts {
    #[serde(rename = "total_posts")]
    pub amount: usize,
}
