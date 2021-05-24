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

    let quality = options.quality.get_value();
    let config: Config = options.into();
    let res = search(config.into()).await;

    res.data.movies.iter().for_each(|m| {
        println!("================================================");
        println!("[{}] {} - {}*", m.year, m.title, m.rating);
        println!("================================================");
        m.torrents.iter().for_each(|t| {
            if t.quality.contains(quality) {
                println!("{}", t.get_magnet_link());
            }
        });
    });
}
