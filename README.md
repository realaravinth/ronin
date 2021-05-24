<div align="center">
  <h1>Ronin</h1>
  <p>
    <strong>Yify Torrents Client</strong>
  </p>

[![dependency status](https://deps.rs/repo/github/realaravinth/ronin/status.svg?style=flat-square)](https://deps.rs/repo/github/realaravinth/ronin)
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg?style=flat-square)](http://www.gnu.org/licenses/agpl-3.0)

</div>

Ronin is a client for accessing Yify Torrents. The website is
ad-invested and has become borderline unusable. I tried everything from
disabling popups, using adblockers and even staying patient while
navigating their website but nothing seemed to work and hence the
motivation to create Ronin.

It talks to Yify's API to fetch movie details and their magnet links.
You can search for movies with their names or their IMDB code for more
successful results. Filters exist for quality, genre and minimum
ratings.

> It's part of a larger meta-search engine that I'm planning to write so
> the CLI tool is probably temporary.

## How to build

- Install Cargo using [rustup](https://rustup.rs/) with:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Clone the repository with:

```
$ git clone https://github.com/realaravinth/ronin
```

- Build with Cargo:

```
$ cd ronin && cargo build --release
```

## Usage

```
ronin 0.1.0
Aravinth Manivannan <realaravinth@batsense.net>
Yify Torrents Client

USAGE:
    ronin-cli [OPTIONS] --name <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -g, --genre <genre>                                        Genre of the film
    -l, --limit <limit>                                          limit search results
    -n, --name <name>
            Name of moive or IMDB code(if you know it)

    -q, --quality <quality>                                      quality: 720p, 1080p, 2160 or 3D
    -r, --rotten-tomatoes-rattings <rotten-tomatoes-rattings>    Minimum ratings of the films
    -s, --sort-by <sort-by>
            Sort by: title year rating peers seeds download_count like_count date_added

    -u, --url <url>
            URL of yify torrents instance accessible to you(if default isn't working)
```
