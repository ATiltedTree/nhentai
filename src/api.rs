use chrono::{serde::ts_seconds, DateTime, Utc};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
#[serde(untagged)]
pub enum Result<T> {
    Ok(T),
    Err { error: String },
}

impl<T> Result<T> {
    pub fn into_std(self) -> crate::Result<T> {
        match self {
            Result::Ok(v) => Ok(v),
            Result::Err { error } => Err(match error.as_str() {
                "does not exist" => crate::Error::DoesNotExist,
                other => todo!("Unkown error reason '{other}'"),
            }),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
pub struct Title {
    pub english: Option<String>,
    pub japanese: Option<String>,
    pub pretty: Option<String>,
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

fn id<'de, D>(de: D) -> std::result::Result<u32, D::Error>
where
    D: serde::Deserializer<'de>,
{
    struct Visitor;

    impl<'de> serde::de::Visitor<'de> for Visitor {
        type Value = u32;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("ID")
        }

        fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as u32)
        }

        fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v as u32)
        }

        fn visit_u32<E>(self, v: u32) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            Ok(v)
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: serde::de::Error,
        {
            v.parse().map_err(|err| serde::de::Error::custom(err))
        }
    }
    de.deserialize_any(Visitor)
}
