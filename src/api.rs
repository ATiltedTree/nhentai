use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
pub struct Title {
    pub english: String,
    pub japanese: String,
    pub pretty: String,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Image {
    pub t: char,
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Images {
    pub pages: Vec<Image>,
    pub cover: Image,
    pub thumbnail: Image,
}

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TagType {
    Tag,
    Language,
    Artist,
    Group,
    Category,
    Parody,
    Character,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Tag {
    pub id: u32,
    #[serde(rename = "type")]
    pub ty: TagType,
    pub name: String,
    pub url: String,
    pub count: u32,
}

#[derive(Clone, Deserialize, Debug)]
pub struct Gallery {
    #[serde(deserialize_with = "id")]
    pub id: u32,
    pub media_id: String,
    pub title: Title,
    pub images: Images,
    pub scanlator: String,
    #[serde(with = "ts_seconds")]
    pub upload_date: DateTime<Utc>,
    pub tags: Vec<Tag>,
    pub num_pages: u32,
    pub num_favorites: u32,
}

fn id<'de, D>(de: D) -> Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("ID")
        }

        fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as u32)
        }

        fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as u32)
        }

        fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse().map_err(|err| serde::de::Error::custom(err))
        }
    }
    de.deserialize_any(Visitor)
}
