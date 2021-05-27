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

use clap::*;
use libyify::search::*;

/// Yify Torrents Client
#[derive(Clap, Clone, Debug)]
#[clap(
    name = "ronin",
    author = "Aravinth Manivannan <realaravinth@batsense.net>",
    version = "0.1.0"
)]
pub struct Options {
    /// Name of moive or IMDB code(if you know it)
    #[clap(short, long)]
    pub name: Option<String>,

    /// quality: 720p, 1080p, 2160 or 3D
    #[clap(short, long)]
    pub quality: Option<Quality>,

    /// URL of yify torrents instance accessible to you(if default isn't working)
    #[clap(short, long)]
    pub url: Option<String>,

    /// limit search results
    #[clap(short, long)]
    pub limit: Option<usize>,

    /// genre of the film
    #[clap(short, long)]
    pub genre: Option<String>,

    /// Minimum ratings of the films
    #[clap(short, long)]
    pub rotten_tomatoes_rattings: Option<usize>,

    /// Sort by: title year rating peers seeds download_count like_count date_added
    #[clap(short, long)]
    pub sort_by: Option<SortBy>,
}

impl From<Options> for Config {
    fn from(o: Options) -> Self {
        Config {
            quality: o.quality,
            limit: o.limit,
            url: o.url,
            rotten_tomatoes_rattings: o.rotten_tomatoes_rattings,
            query_term: o.name,
            genre: o.genre,
            sort_by: o.sort_by,
        }
    }
}
