use crate::{api, client};

#[derive(Clone)]
pub struct Gallery {
    pub(crate) inner: api::Gallery,
    pub(crate) client: client::Client,
}

impl std::fmt::Debug for Gallery {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl Gallery {
    pub fn id(&self) -> u32 {
        self.inner.id
    }

    pub fn date(&self) -> chrono::DateTime<chrono::Utc> {
        self.inner.upload_date
    }

    pub fn tags(&self) -> impl Iterator<Item = Tag<'_>> {
        self.inner.tags.iter().map(|x| Tag { inner: x })
    }

    pub fn page(&self, idx: usize) -> Option<Image<'_>> {
        let media_id = &self.inner.media_id;
        self.inner.images.pages.get(idx).map(|x| Image {
            inner: x,
            media_id,
            ty: ImageTy::Page(idx as u32),
            client: self.client.clone(),
        })
    }

    pub fn pages(&self) -> impl Iterator<Item = Image<'_>> {
        let media_id = &self.inner.media_id;
        self.inner
            .images
            .pages
            .iter()
            .enumerate()
            .map(|(i, x)| Image {
                inner: x,
                media_id,
                ty: ImageTy::Page(i as u32),
                client: self.client.clone(),
            })
    }

    pub fn pages_len(&self) -> u32 {
        self.inner.num_pages
    }

    pub fn favorites(&self) -> u32 {
        self.inner.num_favorites
    }

    pub fn cover(&self) -> Image<'_> {
        Image {
            inner: &self.inner.images.cover,
            media_id: &self.inner.media_id,
            ty: ImageTy::Cover,
            client: self.client.clone(),
        }
    }

    pub fn title(&self) -> Title<'_> {
        Title {
            inner: &self.inner.title,
        }
    }
}

pub struct Title<'a> {
    inner: &'a api::Title,
}

impl<'a> Title<'a> {
    pub fn pretty(&self) -> Option<&str> {
        self.inner
            .pretty
            .as_deref()
            .and_then(|x| (!x.is_empty()).then(|| x))
    }

    pub fn english(&self) -> Option<&str> {
        self.inner
            .english
            .as_deref()
            .and_then(|x| (!x.is_empty()).then(|| x))
    }

    pub fn japanese(&self) -> Option<&str> {
        self.inner
            .japanese
            .as_deref()
            .and_then(|x| (!x.is_empty()).then(|| x))
    }
}

pub use api::TagType;

pub struct Tag<'a> {
    inner: &'a api::Tag,
}

impl<'a> Tag<'a> {
    pub fn id(&self) -> u32 {
        self.inner.id
    }

    pub fn name(&self) -> &str {
        &self.inner.name
    }

    pub fn ty(&self) -> TagType {
        self.inner.ty
    }
}

enum ImageTy {
    Page(u32),
    Cover,
}

pub struct Image<'a> {
    inner: &'a api::Image,
    media_id: &'a str,
    ty: ImageTy,
    client: client::Client,
}

impl<'a> Image<'a> {
    pub fn width(&self) -> u32 {
        self.inner.w
    }

    pub fn height(&self) -> u32 {
        self.inner.h
    }

    pub async fn get(&self) -> crate::Result<reqwest::Response> {
        match self.ty {
            ImageTy::Page(idx) => self.client.page(self.media_id, idx, self.inner.t).await,
            ImageTy::Cover => self.client.cover(self.media_id, self.inner.t).await,
        }
    }

    pub async fn thumbnail(&self) -> crate::Result<reqwest::Response> {
        match self.ty {
            ImageTy::Page(idx) => {
                self.client
                    .page_thumbnail(self.media_id, idx, self.inner.t)
                    .await
            }
            ImageTy::Cover => {
                self.client
                    .cover_thumbnail(self.media_id, self.inner.t)
                    .await
            }
        }
    }
}
