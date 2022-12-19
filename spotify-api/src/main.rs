use dotenvy::dotenv;
use reqwest;
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalUrls {
    spotify: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    name: String,
    external_urls: ExternalUrls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    name: String,
    artists: Vec<Artist>,
    external_urls: ExternalUrls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    name: String,
    href: String,
    popularity: u32,
    album: Album,
    external_urls: ExternalUrls,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Items<T> {
    items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    tracks: Items<Track>,
}
#[tokio::main]
async fn main() {
    dotenv().expect(".Env File Not Found");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage {} <search_query>", args[0]);
        std::process::exit(1);
    }
    let query = &args[1];

    let url = format!("https://api.spotify.com/v1/search?q={query}&type=track,artist");

    let client = reqwest::Client::new();
    let token = env::var("AUTH_TOKEN").unwrap();
    let res = client
        .get(url)
        .header(AUTHORIZATION, format!("Bearer {token}"))
        .header(CONTENT_TYPE, "application/json")
        .header(ACCEPT, "application/json")
        .send()
        .await
        .unwrap();

    match res.status() {
        reqwest::StatusCode::OK => match res.json::<ApiResponse>().await {
            Ok(parsed) => print_tracks(parsed.tracks.items.iter().collect()),
            Err(_) => println!("The response didnt match"),
        },
        reqwest::StatusCode::UNAUTHORIZED => {
            println!("Need to grab a new token");
        }
        _ => {}
    };
}

fn print_tracks(tracks: Vec<&Track>) {
    for track in tracks {
        println!("{:?}", track.name);
        println!("{:?}", track.album.name);
        println!(
            "{:?}",
            track
                .album
                .artists
                .iter()
                .map(|artist| artist.name.to_string())
                .collect::<String>()
        );
        println!("{:?}", track.external_urls);
        println!("---------------------------------------");
    }
}
