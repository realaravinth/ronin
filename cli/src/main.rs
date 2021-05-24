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
        print!("[{}], {} - {}*", m.year, m.title, m.rating);
        m.torrents.iter().for_each(|t| {
            if t.quality.contains(quality) {
                println!(" {}\n{}", t.quality, t.get_magnet_link());
            }
        });
        println!("================================================");
    });
}
