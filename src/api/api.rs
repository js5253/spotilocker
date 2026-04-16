
use anyhow::anyhow;
use axum::extract::State;
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct SpotifyShowResponse {
    #[serde(rename = "href")]
    pub href: Option<String>,
    #[serde(rename = "limit")]
    pub limit: u32,
    #[serde(rename = "next")]
    pub next: Option<String>,
    #[serde(rename = "offset")]
    pub offset: u32,
    #[serde(rename = "previous")]
    pub previous: Option<String>,
    #[serde(rename = "total")]
    pub total: u32,
    #[serde(rename = "items")]
    pub items: Vec<Track>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Track {
    #[serde(rename = "added_at")]
    pub added_at: Option<String>,

    #[serde(rename = "track")]
    pub track: TrackDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackDetails {
    #[serde(rename = "album")]
    pub album: Album,

    #[serde(rename = "artists")]
    pub artists: Vec<Artist>,

    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,

    #[serde(rename = "disc_number")]
    pub disc_number: u32,

    #[serde(rename = "duration_ms")]
    pub duration_ms: u32,

    #[serde(rename = "explicit")]
    pub explicit: bool,

    #[serde(rename = "external_ids")]
    pub external_ids: Option<ExternalIds>,

    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,

    #[serde(rename = "href")]
    pub href: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "is_playable")]
    pub is_playable: bool,

    #[serde(rename = "linked_from")]
    pub linked_from: Option<LinkData>,

    #[serde(rename = "restrictions")]
    pub restrictions: Option<Restrictions>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "popularity")]
    pub popularity: u32,

    #[serde(rename = "preview_url")]
    pub preview_url: Option<String>,

    #[serde(rename = "track_number")]
    pub track_number: u32,

    #[serde(rename = "type")]
    pub track_type: String, // "track"

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "is_local")]
    pub is_local: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Album {
    #[serde(rename = "album_type")]
    pub album_type: String,

    #[serde(rename = "total_tracks")]
    pub total_tracks: u32,

    #[serde(rename = "available_markets")]
    pub available_markets: Vec<String>,

    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,

    #[serde(rename = "href")]
    pub href: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "images")]
    pub images: Vec<Image>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "release_date")]
    pub release_date: Option<String>,

    #[serde(rename = "release_date_precision")]
    pub release_date_precision: String,

    #[serde(rename = "restrictions")]
    pub restrictions: Option<Restrictions>,

    #[serde(rename = "type")]
    pub item_type: String, // e.g., "album"

    #[serde(rename = "uri")]
    pub uri: String,

    #[serde(rename = "artists")]
    pub artists: Vec<Artist>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Artist {
    #[serde(rename = "external_urls")]
    pub external_urls: ExternalUrls,

    #[serde(rename = "href")]
    pub href: String,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "type")]
    pub artist_type: String, // e.g., "artist"

    #[serde(rename = "uri")]
    pub uri: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "height")]
    pub height: u32,

    #[serde(rename = "width")]
    pub width: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalUrls {
    #[serde(rename = "spotify")]
    pub spotify: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExternalIds {
    #[serde(rename = "isrc")]
    pub isrc: Option<String>,

    #[serde(rename = "ean")]
    pub ean: Option<String>,

    #[serde(rename = "upc")]
    pub upc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Restrictions {
    #[serde(rename = "reason")]
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LinkData {
    // Could be an object containing references to other resources (e.g., album, track)
    // For now, we leave empty — can be expanded later
    // You can add fields like href, id, etc. as needed
}
#[derive(Debug, Deserialize)]
pub struct SpotifyTokenResponse {
    access_token: String,
    token_type: String,
    expires_in: String
}

pub async fn get_liked_songs(client: &reqwest::Client, page: usize, token: String) -> anyhow::Result<SpotifyShowResponse> {
    let offset = page * 20;
    let req = client.get(format!("https://api.spotify.com/v1/me/tracks?offset={}&limit=20", offset)).bearer_auth(token).send().await?;
    let res = req.error_for_status().map_err(|e| anyhow!(e.to_string()))?.json::<SpotifyShowResponse>().await?;


    Ok(res)
}



pub async fn fetch_token(app_state: &AppState) -> anyhow::Result<String>{
    let params =format!("?grant_type=client_credentials&client_id={}&client_secret={}", app_state.client_id, app_state.client_secret);
    let req = app_state.http_client.post(format!("https://accounts.spotify.com/api/token{}", params)).header("Content-Type", "application/x-www-form-urlencoded").send().await?;

    let res = req.error_for_status().map_err(|e| anyhow!(e.to_string()))?.json::<SpotifyTokenResponse>().await?;

    Ok(res.access_token)

}
