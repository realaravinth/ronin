use std::{fmt, str::FromStr};

use reqwest::ClientBuilder;
use serde::*;
use url::Url;

use super::*;

#[derive(Serialize, Deserialize)]
pub struct Data {
    pub movie_count: usize,
    pub limit: usize,
    pub page_number: usize,
    pub movies: Vec<Movie>,
}

#[derive(Serialize, Deserialize)]
pub struct ListResult {
    pub status: String,
    pub status_message: String,
    #[serde(rename(deserialize = "@meta"))]
    pub meta: Meta,
    pub data: Data,
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

pub trait Value {
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

#[derive(Clone)]
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

impl fmt::Debug for Quality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Quality:")
            .field("x", &self.get_value())
            .finish()
    }
}

impl FromStr for Quality {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "720p" => Ok(Self::HD),
            "1080p" => Ok(Self::FHD),
            "3D" => Ok(Self::ThreeD),
            "2160" => Ok(Self::FourK),
            _ => Err("Quality doesn't exist"),
        }
    }
}

pub struct ListUrl(pub Url);

impl Default for ListUrl {
    fn default() -> Self {
        ListUrl::new("https://yts.unblocked.name/api/v2/list_movies.jsonp")
    }
}

impl ListUrl {
    pub fn new(url: &str) -> Self {
        let url = Url::parse(url).unwrap();
        ListUrl(url)
    }

    fn limit(&mut self, limit: usize) {
        self.0.query_pairs_mut().append_pair(
            //self.0
            //.set_query(Some(&format!("limit={}", limit.to_string())));
            "limit",
            &limit.to_string(),
        );
    }

    fn with_query(&mut self, query_term: String) {
        self.0.query_pairs_mut().append_pair(
            //self.0
            //.set_query(Some(&format!("query_term={}", query_term)));
            "query_term",
            &query_term,
        );
    }

    fn rotten_tomatoes_rattings(&mut self, ratings: usize) {
        self.0.query_pairs_mut().append_pair(
            //self.0
            //.set_query(Some(&format!("with_rt_ratings={}", ratings.to_string())));
            "with_rt_ratings",
            &ratings.to_string(),
        );
    }

    fn genere(&mut self, genere: &str) {
        self.0.query_pairs_mut().append_pair(
            //self.0.set_query(Some(&format!("genere={}", genere)));
            "genere", genere,
        );
    }

    fn sort_by(&mut self, sort_by: SortBy) {
        self.0.query_pairs_mut().append_pair(
            //        self.0
            //.set_query(Some(&format!("sort_by={}", sort_by.get_value())));
            "sort_by",
            sort_by.get_value(),
        );
    }

    fn quality(&mut self, quality: Quality) {
        self.0
            .query_pairs_mut()
            .append_pair("quality", quality.get_value());
        //        self.0
        //            .set_query(Some(&format!("quality={}", quality.get_value())));
    }
}

#[derive(Default)]
pub struct Config {
    pub quality: Option<Quality>,
    pub url: Option<String>,
    pub sort_by: Option<SortBy>,
    pub genere: Option<String>,
    pub rotten_tomatoes_rattings: Option<usize>,
    pub limit: Option<usize>,
    pub query_term: Option<String>,
}

impl From<Config> for ListUrl {
    fn from(c: Config) -> Self {
        let mut list_url = ListUrl::default();
        if let Some(url) = c.url {
            list_url = ListUrl::new(&url);
        };

        if let Some(query_term) = c.query_term {
            list_url.with_query(query_term);
        }

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
pub async fn search(url: ListUrl) -> ListResult {
    let client = ClientBuilder::default()
        .user_agent(crate::USER_AGENT)
        .use_rustls_tls()
        .build()
        .unwrap();
    let resp = client.get(url.0).send().await.unwrap();
    //    println!("{:#?}", resp);
    //    println!("{:#?}", resp);
    assert!(resp.status().is_success());
    //let res = resp.text().await.unwrap();
    //println!("{:?}", res);
    //let res: serde_json::Value = resp.json().await.unwrap();
    let res: ListResult = resp.json().await.unwrap();
    //    println!("{:?}", res);
    //    let mut res = client .get(&url.0.to_string()) .insert_header(("User-Agent", crate::USER_AGENT)) .send() .await .unwrap();

    res
}
