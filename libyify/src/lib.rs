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

mod details;
mod search;

pub const USER_AGENT: &str = "Chromium/55.0 (Windows) Cockroach";

#[derive(Serialize, Deserialize)]
pub struct Movie {
    id: usize,
    url: String,
    title: String,
    title_english: String,
    title_long: String,
    slug: String,
    year: usize,
    rating: usize,
    generes: Vec<String>,
    summary: String,
    description_full: String,
    yt_trailer_code: String,
    language: String,
    mpa_rating: String,
    background_image: String,
    background_image_original: String,
    small_cover_image: String,
    medium_cover_image: String,
    large_cover_image: String,
    state: String,
    torrents: Vec<Torrent>,
}

#[derive(Serialize, Deserialize)]
pub struct Torrent {
    url: String,
    hash: String,
    quality: String,
    #[serde(rename(serialize = "type"))]
    rip_type: String,
    seeds: usize,
    peers: usize,
    size: String,
    size_bytes: usize,
    date_uploaded: String,
    date_uploaded_unix: usize,
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
