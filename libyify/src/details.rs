/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

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
    let client = reqwest::ClientBuilder::default()
        .user_agent(crate::USER_AGENT)
        .use_rustls_tls()
        .build()
        .unwrap();
    let res: ListResult = client
        .get(url.0)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    //    let mut res = client .get(&url.0.to_string()) .insert_header(("User-Agent", crate::USER_AGENT)) .send() .await .unwrap();

    res
}
