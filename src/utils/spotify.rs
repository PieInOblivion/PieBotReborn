use base64_light::base64_encode;
use serde_json::Value;

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::RwLock;

use serenity::prelude::Context;

use crate::utils::structs::{BotData, Song};

struct SpotifyToken {
    token: String,
    token_birth: SystemTime,
    token_expires_in_sec: u64,
}

#[derive(Clone)]
pub struct Spotify {
    id: Arc<str>,
    secret: Arc<str>,
    token: Arc<RwLock<SpotifyToken>>,
}

impl Spotify {
    pub async fn new(id: String, secret: String) -> Spotify {
        Spotify {
            id: Arc::from(id),
            secret: Arc::from(secret),
            token: Arc::new(RwLock::new(SpotifyToken {
                token: String::new(),
                token_birth: SystemTime::now(),
                token_expires_in_sec: 0, // Token will refresh on first use
            })),
        }
    }

    async fn get_token(&self, ctx: &Context) -> String {
        let mut token_info = self.token.write().await;
        let sec_since_refresh = SystemTime::now()
            .duration_since(token_info.token_birth)
            .unwrap()
            .as_secs();

        // 10 second buffer or empty token
        if sec_since_refresh + 10 > token_info.token_expires_in_sec || token_info.token.is_empty() {
            let (new_token, expires) = Self::get_token_new(ctx, &self.id, &self.secret)
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

    async fn get_token_new(ctx: &Context, id: &str, secret: &str) -> Option<(String, u64)> {
        let auth_url = "https://accounts.spotify.com/api/token";
        let auth = base64_encode(format!("{}:{}", id, secret).as_str());
        let auth_code = format!("Basic {}", auth);

        let data = ctx.data::<BotData>();

        let response = data
            .http
            .post(auth_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Authorization", &auth_code)
            .body("grant_type=client_credentials")
            .send()
            .await
            .ok()?;

        let json: Value = response.json().await.ok()?;

        Some((
            json["access_token"].as_str()?.to_string(),
            json["expires_in"].as_u64()?,
        ))
    }

    pub async fn get_album_tracks(&self, ctx: &Context, id: &str) -> Option<VecDeque<Song>> {
        let mut next_url = format!(
            "https://api.spotify.com/v1/albums/{}/tracks?limit=50&offset=0",
            id
        );

        let mut album: VecDeque<Song> = VecDeque::new();

        loop {
            let json = Self::https_req(self, ctx, &next_url).await?;

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
                    title: Arc::from(format!("{} {}", artists, title)),
                });
            }

            next_url = json["next"].as_str().get_or_insert("").to_string();

            if next_url.is_empty() {
                break;
            }
        }

        Some(album)
    }

    pub async fn get_playlist_tracks(&self, ctx: &Context, id: &str) -> Option<VecDeque<Song>> {
        let mut next_url = format!(
            "https://api.spotify.com/v1/playlists/{}/tracks?limit=100&offset=0",
            id
        );

        let mut playlist: VecDeque<Song> = VecDeque::new();

        loop {
            let json = Self::https_req(self, ctx, &next_url).await?;

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
                    title: Arc::from(format!("{} {}", artists, title)),
                });
            }

            next_url = json["next"].as_str().get_or_insert("").to_string();

            if next_url.is_empty() {
                break;
            }
        }

        Some(playlist)
    }

    pub async fn get_track(&self, ctx: &Context, id: &str) -> Option<Song> {
        let url = format!("https://api.spotify.com/v1/tracks/{}", id);

        let json = Self::https_req(self, ctx, &url).await?;

        let title = json["name"].as_str()?.to_string();

        let artists = json["artists"]
            .as_array()?
            .iter()
            .map(|artist| artist["name"].as_str().unwrap())
            .collect::<Vec<&str>>()
            .join(" ");

        Some(Song {
            id: None,
            title: Arc::from(format!("{} {}", artists, title)),
        })
    }

    async fn https_req(&self, ctx: &Context, url: &str) -> Option<serde_json::Value> {
        let token = Self::get_token(self, ctx).await;

        let data = ctx.data::<BotData>();

        let response = data
            .http
            .get(url)
            .header("Authorization", &format!("Bearer {}", token))
            .send()
            .await
            .ok()?;

        response.json().await.ok()
    }
}
