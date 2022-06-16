use reqwest::StatusCode;

use crate::Gallery;

fn get_type(file_type: char) -> &'static str {
    match file_type {
        'j' => "jpg",
        'p' => "png",
        _ => "gif",
    }
}

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Self::new(None)
    }
}

impl Client {
    pub fn new(cookie: Option<&str>) -> Self {
        use reqwest::header;

        let mut headers = header::HeaderMap::new();
        if let Some(cookie) = cookie {
            let mut cook = header::HeaderValue::from_str(cookie).unwrap();
            cook.set_sensitive(true);
            headers.insert(header::COOKIE, cook);
        }

        // get a client builder
        let client = reqwest::Client::builder()
            .default_headers(headers)
            // .user_agent("Mozilla/5.0 (Windows NT 10.0; rv:91.0) Gecko/20100101 Firefox/91.0")
            .build()
            .unwrap();

        Self {
            // client: reqwest::Client::new(),
            client,
        }
    }

    pub async fn gallery(&self, id: u32) -> crate::Result<Gallery> {
        let inner = self
            .client
            .get(format!("https://nhentai.net/api/gallery/{id}"))
            .send()
            .await?;

        if inner.status() == StatusCode::NOT_FOUND {
            return Err(crate::Error::NotFound(id));
        }
        let inner = inner.json().await?;

        Ok(Gallery {
            inner,
            client: self.clone(),
        })
    }

    pub(crate) async fn cover(
        &self,
        media_id: &str,
        file_type: char,
    ) -> crate::Result<reqwest::Response> {
        let url = format!(
            "https://t.nhentai.net/galleries/{}/cover.{}",
            media_id,
            get_type(file_type)
        );

        Ok(self.client.get(url).send().await?)
    }

    pub(crate) async fn cover_thumbnail(
        &self,
        media_id: &str,
        file_type: char,
    ) -> crate::Result<reqwest::Response> {
        let url = format!(
            "https://t.nhentai.net/galleries/{}/thumb.{}",
            media_id,
            get_type(file_type)
        );
        Ok(self.client.get(url).send().await?)
    }

    pub(crate) async fn page(
        &self,
        media_id: &str,
        number: u32,
        file_type: char,
    ) -> crate::Result<reqwest::Response> {
        let url = format!(
            "https://i.nhentai.net/galleries/{}/{}.{}",
            media_id,
            number + 1,
            get_type(file_type)
        );

        Ok(self.client.get(url).send().await?)
    }

    pub(crate) async fn page_thumbnail(
        &self,
        media_id: &str,
        number: u32,
        file_type: char,
    ) -> crate::Result<reqwest::Response> {
        let url = format!(
            "https://t.nhentai.net/galleries/{}/{}t.{}",
            media_id,
            number + 1,
            get_type(file_type)
        );

        Ok(self.client.get(url).send().await?)
    }
}
