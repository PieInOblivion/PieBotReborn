use hyper::body::to_bytes;
use hyper::{Body, Client, Request};
use hyper_rustls::HttpsConnectorBuilder;

use serde_json::{from_slice, Value};

use base64_light::base64_encode;

use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::prelude::TypeMapKey;

use songbird::tracks::TrackHandle;

use std::collections::HashMap;
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

// TODO: wrap token in arc rwlock. Maybe make token new struct
// TODO: Decide; individual functions getting passed Spotify, queries part of Spotify. Latter..
pub struct Spotify {
    id: String,
    secret: String,
    token: String,
    token_birth: SystemTime,
    token_expires_in_sec: u64,
}

impl Spotify {
    pub async fn new(id: String, secret: String) -> Spotify {
        let (new_token, expires) = Self::get_token_new(id.clone(), secret.clone())
            .await
            .unwrap();
        return Spotify {
            id,
            secret,
            token: new_token,
            token_birth: SystemTime::now(),
            token_expires_in_sec: expires,
        };
    }
    pub async fn get_token(&mut self) -> String {
        let sec_since_refresh = SystemTime::now()
            .duration_since(self.token_birth)
            .unwrap()
            .as_secs();
        // TODO: modify to return options instead of unwraps
        // 10 second buffer
        if sec_since_refresh + 10 > self.token_expires_in_sec {
            let (new_token, expires) = Self::get_token_new(self.id.clone(), self.secret.clone())
                .await
                .unwrap();
            self.token = new_token;
            self.token_birth = SystemTime::now();
            self.token_expires_in_sec = expires;
            self.token.clone()
        } else {
            self.token.clone()
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
            .method(hyper::Method::POST)
            .uri(auth_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", auth_code)
            .body(hyper::Body::from("grant_type=client_credentials"))
            .ok()?;

        let res = client.request(req).await.ok()?;

        let body = to_bytes(res.into_body()).await.ok()?;

        let json: Value = from_slice(&body).ok()?;

        Some((
            json["access_token"].to_string(),
            json["expires_in"].as_u64().unwrap(),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct SerProps {
    pub bot_text_channel: ChannelId,
    pub request_queue: Vec<Song>,
    pub playlist_queue: Vec<Song>,
    pub playing: Option<Song>,
    pub playing_handle: Option<TrackHandle>,
}

impl SerProps {
    pub fn new(channel_id: ChannelId) -> SerProps {
        return SerProps {
            bot_text_channel: channel_id,
            request_queue: Vec::new(),
            playlist_queue: Vec::new(),
            playing: None,
            playing_handle: None,
        };
    }

    pub fn playlist_queue_shuffle(&mut self) {
        self.playlist_queue.shuffle(&mut thread_rng());
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
