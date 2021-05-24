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
use libyify::search::Config;

/// Yify Torrents Client
#[derive(Clap, Clone, Debug)]
#[clap(
    name = "ronin",
    author = "Aravinth Manivannan <realaravinth@batsense.net>"
)]
pub struct Options {
    /// Name of the person to greet
    #[clap(short, long)]
    pub name: String,

    #[clap(short, long)]
    pub quality: libyify::search::Quality,

    #[clap(short, long)]
    pub url: Option<String>,

    #[clap(short, long)]
    pub limit: Option<usize>,
}

impl From<Options> for Config {
    fn from(o: Options) -> Self {
        Config {
            quality: Some(o.quality),
            limit: o.limit,
            url: o.url,
            rotten_tomatoes_rattings: None,
            query_term: Some(o.name),
            genere: None,
            sort_by: None,
        }
    }
}
