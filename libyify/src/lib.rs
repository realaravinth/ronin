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

pub mod details;
pub mod search;

pub const USER_AGENT: &str = "Chromium/55.0 (Windows) Cockroach";

#[derive(Serialize, Deserialize)]
pub struct Movie {
    pub id: usize,
    pub url: String,
    pub title: String,
    pub title_english: String,
    pub title_long: String,
    pub slug: String,
    pub year: usize,
    pub rating: f32,
    pub genres: Vec<String>,
    pub summary: String,
    pub description_full: String,
    pub yt_trailer_code: String,
    pub language: String,
    pub mpa_rating: String,
    pub background_image: String,
    pub background_image_original: String,
    pub small_cover_image: String,
    pub medium_cover_image: String,
    pub large_cover_image: String,
    pub state: String,
    pub torrents: Vec<Torrent>,
}

#[derive(Serialize, Deserialize)]
pub struct Torrent {
    pub url: String,
    pub hash: String,
    pub quality: String,
    #[serde(rename(deserialize = "type"))]
    pub rip_type: String,
    pub seeds: usize,
    pub peers: usize,
    pub size: String,
    pub size_bytes: usize,
    pub date_uploaded: String,
    pub date_uploaded_unix: usize,
}

impl Torrent {
    pub fn get_magnet_link(&self) -> String {
        format!("magnet:?xt=urn:btih:{}&dn=Url+Encoded+Movie+Name&tr=http://track.one:1234/announce&tr=udp://track.two:80", self.hash)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Meta {
    server_time: usize,
    server_timezone: String,
    api_version: usize,
    execution_time: String,
}
