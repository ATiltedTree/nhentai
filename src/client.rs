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
            .build()
            .unwrap();

        Self { client }
    }

    pub async fn gallery(&self, id: u32) -> crate::Result<Gallery> {
        let inner: crate::api::Result<crate::api::Gallery> = self
            .client
            .get(format!("https://nhentai.net/api/gallery/{id}"))
            .send()
            .await?
            .json()
            .await?;

        Ok(Gallery {
            inner: inner.into_std()?,
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
