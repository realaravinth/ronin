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

use clap::Clap;
use libyify::search::*;

mod options;

#[actix_rt::main]
async fn main() {
    let options = options::Options::parse();

    let quality = options.quality.clone();
    let new_options = options.clone();
    let config: Config = options.into();
    let res = search(config.into()).await;

    res.data.movies.iter().for_each(|m| {
        println!("================================================");
        println!("[{}] {} - {}*", m.year, m.title, m.rating);
        println!("================================================");

        if new_options.description {
            println!("Description:");
            println!("------------");
            println!("{}\n", m.description_full);
        }

        if new_options.websites {
            println!("Websites:");
            println!("---------");
            println!("Yify: {}", m.url);
            println!("IMDB {}\n", m.get_imdb_url());
        }

        if m.torrents.len() > 0 {
            println!("Torrents:");
            println!("---------");
        }
        m.torrents.iter().for_each(|t| {
            if quality.is_some() {
                if t.quality.contains(quality.as_ref().unwrap().get_value()) {
                    println!("[{}]: {}", t.quality, t.get_magnet_link());
                }
            } else {
                println!("[{}]: {}", t.quality, t.get_magnet_link());
            }
        });
    });
}
