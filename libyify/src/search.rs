use awc::Client;
use serde::*;
use url::Url;

use super::*;

#[derive(Serialize, Deserialize)]
pub struct Data {
    movie_count: usize,
    limit: usize,
    page_number: usize,
    movies: Vec<Movie>,
}

#[derive(Serialize, Deserialize)]
pub struct ListResult {
    status: String,
    status_message: String,
    #[serde(rename(serialize = "@meta"))]
    meta: Meta,
    data: Data,
}
pub enum SortBy {
    Title,
    Year,
    Rating,
    Peers,
    Seeds,
    DownloadCount,
    LikeCount,
    DateAdded,
}

trait Value {
    type Result;
    fn get_value(&self) -> Self::Result;
}

impl Value for SortBy {
    type Result = &'static str;

    fn get_value(&self) -> Self::Result {
        match *self {
            SortBy::Title => "title",
            SortBy::Year => "year",
            SortBy::Rating => "rating",
            SortBy::Peers => "peers",
            SortBy::Seeds => "seeds",
            SortBy::DownloadCount => "download_count",
            SortBy::LikeCount => "like_count",
            SortBy::DateAdded => "date_added",
        }
    }
}

pub enum Quality {
    HD,
    FHD,
    FourK,
    ThreeD,
}

impl Value for Quality {
    type Result = &'static str;

    fn get_value(&self) -> Self::Result {
        match *self {
            Self::HD => "720p",
            Self::FHD => "1080p",
            Self::ThreeD => "3D",
            Self::FourK => "2160",
        }
    }
}

pub struct ListUrl(pub Url);

impl Default for ListUrl {
    fn default() -> Self {
        ListUrl::new("https://yts.unblocked.name/api/v2/list_movies.json")
    }
}

impl ListUrl {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(url).unwrap();
        ListUrl(url)
    }

    fn limit(&mut self, limit: usize) {
        self.0
            .set_query(Some(&format!("limit={}", limit.to_string())));
    }

    fn rotten_tomatoes_rattings(&mut self, ratings: usize) {
        self.0
            .set_query(Some(&format!("with_rt_ratings={}", ratings.to_string())));
    }

    fn genere(&mut self, genere: &str) {
        self.0.set_query(Some(&format!("genere={}", genere)));
    }

    fn sort_by(&mut self, sort_by: SortBy) {
        self.0
            .set_query(Some(&format!("sort_by={}", sort_by.get_value())));
    }

    fn quality(&mut self, quality: Quality) {
        self.0
            .set_query(Some(&format!("quality={}", quality.get_value())));
    }
}

pub struct Config {
    quality: Option<Quality>,
    url: Option<String>,
    sort_by: Option<SortBy>,
    genere: Option<String>,
    rotten_tomatoes_rattings: Option<usize>,
    limit: Option<usize>,
}

impl From<Config> for ListUrl {
    fn from(c: Config) -> Self {
        let mut list_url = ListUrl::default();
        if let Some(url) = c.url {
            list_url = ListUrl::new(&url);
        };

        if let Some(sort_by) = c.sort_by {
            list_url.sort_by(sort_by);
        }

        if let Some(genere) = c.genere {
            list_url.genere(&genere);
        }

        if let Some(rotten_tomatoes_rattings) = c.rotten_tomatoes_rattings {
            list_url.rotten_tomatoes_rattings(rotten_tomatoes_rattings);
        }

        if let Some(quality) = c.quality {
            list_url.quality(quality);
        }

        if let Some(limit) = c.limit {
            list_url.limit(limit);
        }

        list_url
    }
}
async fn search(url: ListUrl) -> ListResult {
    let mut res = Client::default()
        .get(&url.0.to_string())
        .header("User-Agent", crate::USER_AGENT)
        .send()
        .await
        .unwrap();

    let val: ListResult = res.json().await.unwrap();
    val
}
