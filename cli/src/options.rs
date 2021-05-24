use clap::*;
use libyify::search::Config;

/// Yify Torrents Client
#[derive(Clap, Clone, Debug)]
#[clap(name = "ronin")]
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
