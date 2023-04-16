use hyper::body::to_bytes;
use hyper::client::HttpConnector;
use hyper::Method;
use hyper::{Body, Client, Request};
use hyper_rustls::{HttpsConnector, HttpsConnectorBuilder};

use serde_json::{from_slice, Value};

use base64_light::base64_encode;

use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::prelude::TypeMapKey;

use songbird::tracks::TrackHandle;

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::time::SystemTime;

use rand::seq::SliceRandom;
use rand::thread_rng;

use tokio::sync::RwLock;

pub struct AllSerProps;

impl TypeMapKey for AllSerProps {
    type Value = HashMap<GuildId, Arc<RwLock<SerProps>>>;
}

impl TypeMapKey for Spotify {
    type Value = Spotify;
}

struct SpotifyToken {
    token: String,
    token_birth: SystemTime,
    token_expires_in_sec: u64,
}

#[derive(Clone)]
pub struct Spotify {
    id: String,
    secret: String,
    token: Arc<RwLock<SpotifyToken>>,
}

impl Spotify {
    pub async fn new(id: String, secret: String) -> Spotify {
        let (new_token, expires) = Self::get_token_new(id.clone(), secret.clone())
            .await
            .unwrap();
        return Spotify {
            id,
            secret,
            token: Arc::new(RwLock::new(SpotifyToken {
                token: new_token,
                token_birth: SystemTime::now(),
                token_expires_in_sec: expires,
            })),
        };
    }

    // Will panic if new token cannot be retrieved
    async fn get_token(&mut self) -> String {
        let mut token_info = self.token.write().await;
        let sec_since_refresh = SystemTime::now()
            .duration_since(token_info.token_birth)
            .unwrap()
            .as_secs();

        // 10 second buffer
        if sec_since_refresh + 10 > token_info.token_expires_in_sec {
            let (new_token, expires) = Self::get_token_new(self.id.clone(), self.secret.clone())
                .await
                .unwrap();
            token_info.token = new_token;
            token_info.token_birth = SystemTime::now();
            token_info.token_expires_in_sec = expires;
            token_info.token.clone()
        } else {
            token_info.token.clone()
        }
    }

    async fn get_token_new(id: String, secret: String) -> Option<(String, u64)> {
        let auth_url = "https://accounts.spotify.com/api/token";
        let auth = base64_encode(format!("{}:{}", id, secret).as_str());
        let auth_code = format!("Basic {}", auth);

        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http2()
            .build();

        let client = Client::builder().build::<_, Body>(https);

        let req = Request::builder()
            .method(Method::POST)
            .uri(auth_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", auth_code)
            .body(hyper::Body::from("grant_type=client_credentials"))
            .ok()?;

        let res = client.request(req).await.ok()?;

        let body = to_bytes(res.into_body()).await.ok()?;

        let json: Value = from_slice(&body).ok()?;

        Some((
            json["access_token"].as_str()?.to_string(),
            json["expires_in"].as_u64()?,
        ))
    }

    pub async fn get_album_tracks(&mut self, id: &String) -> Option<VecDeque<Song>> {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http2()
            .build();

        let client = Client::builder().build::<_, Body>(https);

        let mut next_url = format!(
            "https://api.spotify.com/v1/albums/{}/tracks?limit=50&offset=0",
            id
        );

        let mut album: VecDeque<Song> = VecDeque::new();

        loop {
            let json = Self::https_req(self, &client, next_url).await?;

            for item in json["items"].as_array()?.iter() {
                let title = item["name"].as_str()?.to_string();

                let artists = item["artists"]
                    .as_array()?
                    .iter()
                    .map(|artist| artist["name"].as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join(" ");

                album.push_back(Song {
                    id: None,
                    title: format!("{} {}", artists, title),
                });
            }

            next_url = json["next"].as_str().get_or_insert("").to_string();

            if next_url == "" {
                break;
            }
        }

        Some(album)
    }

    pub async fn get_playlist_tracks(&mut self, id: &String) -> Option<VecDeque<Song>> {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http2()
            .build();

        let client = Client::builder().build::<_, Body>(https);

        let mut next_url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks?limit=100&offset=0",
            id
        );

        let mut playlist: VecDeque<Song> = VecDeque::new();

        loop {
            let json = Self::https_req(self, &client, next_url).await?;

            for item in json["items"].as_array()?.iter() {
                let title = item["track"]["name"].as_str()?.to_string();

                let artists = item["track"]["artists"]
                    .as_array()?
                    .iter()
                    .map(|artist| artist["name"].as_str().unwrap())
                    .collect::<Vec<&str>>()
                    .join(" ");

                playlist.push_back(Song {
                    id: None,
                    title: format!("{} {}", artists, title),
                });
            }

            next_url = json["next"].as_str().get_or_insert("").to_string();

            if next_url == "" {
                break;
            }
        }

        Some(playlist)
    }

    pub async fn get_track(&mut self, id: &String) -> Option<Song> {
        let https = HttpsConnectorBuilder::new()
            .with_native_roots()
            .https_only()
            .enable_http2()
            .build();

        let client = Client::builder().build::<_, Body>(https);

        let url = format!("https://api.spotify.com/v1/tracks/{}", id);

        let json = Self::https_req(self, &client, url).await?;

        let title = json["name"].as_str()?.to_string();

        let artists = json["artists"]
            .as_array()?
            .iter()
            .map(|artist| artist["name"].as_str().unwrap())
            .collect::<Vec<&str>>()
            .join(" ");

        Some(Song {
            id: None,
            title: format!("{} {}", artists, title),
        })
    }

    async fn https_req(
        &mut self,
        client: &hyper::Client<HttpsConnector<HttpConnector>>,
        next_url: String,
    ) -> Option<serde_json::Value> {
        let token = Self::get_token(self).await;

        let req = Request::builder()
            .method(Method::GET)
            .uri(next_url)
            .header("Authorization", format!("Bearer {}", token))
            .body(hyper::Body::empty())
            .ok()?;

        let res = client.request(req).await.ok()?;

        let body = to_bytes(res.into_body()).await.ok()?;

        Some(from_slice(&body).ok()?)
    }
}

#[derive(Clone, Debug)]
pub struct SerProps {
    pub bot_text_channel: ChannelId,
    pub request_queue: VecDeque<Song>,
    pub playlist_queue: VecDeque<Song>,
    pub playing: Option<Song>,
    pub playing_handle: Option<TrackHandle>,
}

impl SerProps {
    pub fn new(channel_id: ChannelId) -> SerProps {
        return SerProps {
            bot_text_channel: channel_id,
            request_queue: VecDeque::new(),
            playlist_queue: VecDeque::new(),
            playing: None,
            playing_handle: None,
        };
    }

    pub fn playlist_queue_shuffle(&mut self) {
        self.playlist_queue
            .make_contiguous()
            .shuffle(&mut thread_rng());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Song {
    pub id: Option<String>,
    pub title: String,
}

#[derive(Debug)]
pub struct SongFilterResult {
    pub yt_id: Option<String>,
    pub yt_list: Option<String>,
    pub spot_track: Option<String>,
    pub spot_list: Option<String>,
    pub spot_album: Option<String>,
    pub search_needed: bool,
}

// TODO: Doesn't work because of lifetimes.
// #[macro_export]
// macro_rules! get_serprops {
//     ($ctx:expr, $gid:expr) => {{
//         let allserprops = {
//             let data_read = $ctx.data.read().await;
//             data_read.get::<AllSerProps>().unwrap().clone()
//         };

//         let mut wait_write = allserprops.write().await;
//         let serprops = wait_write.get_mut($gid).unwrap();
//         std::rc::Rc::new((serprops, wait_write, allserprops))
//     }};
// }
