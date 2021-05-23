use awc::Client;
use serde::*;
use url::Url;

use super::*;

#[derive(Serialize, Deserialize)]
pub struct Data {
    movie: Movie,
}

#[derive(Serialize, Deserialize)]
pub struct ListResult {
    status: String,
    status_message: String,
    #[serde(rename(serialize = "@meta"))]
    meta: Meta,
    data: Data,
}

pub struct DetailsUrlBuilder(pub Url);

pub struct DetailsUrl(pub Url);

impl Default for DetailsUrlBuilder {
    fn default() -> Self {
        DetailsUrlBuilder::new("https://yts.unblocked.name/api/v2/movie_details.json")
    }
}

impl DetailsUrlBuilder {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(url).unwrap();
        DetailsUrlBuilder(url)
    }

    fn movie_id(mut self, movie_id: usize) -> DetailsUrl {
        self.0
            .set_query(Some(&format!("movie_id={}", movie_id.to_string())));

        DetailsUrl(self.0)
    }

    fn with_images(&mut self, with_images: bool) {
        self.0
            .set_query(Some(&format!("with_images={}", with_images)));
    }

    fn with_cast(&mut self, with_cast: bool) {
        self.0.set_query(Some(&format!("with_cast={}", with_cast)));
    }
}

pub struct Config {
    with_cast: Option<bool>,
    with_images: Option<bool>,
    url: Option<String>,
    movie_id: usize,
}

impl From<Config> for DetailsUrl {
    fn from(c: Config) -> Self {
        let mut details_url = DetailsUrlBuilder::default();
        if let Some(url) = c.url {
            details_url = DetailsUrlBuilder::new(&url);
        };

        if let Some(with_cast) = c.with_cast {
            details_url.with_cast(with_cast);
        }

        if let Some(with_images) = c.with_images {
            details_url.with_images(with_images);
        }

        details_url.movie_id(c.movie_id)
    }
}
async fn search(url: DetailsUrl) -> ListResult {
    let mut res = Client::default()
        .get(&url.0.to_string())
        .header("User-Agent", crate::USER_AGENT)
        .send()
        .await
        .unwrap();

    let val: ListResult = res.json().await.unwrap();
    val
}
