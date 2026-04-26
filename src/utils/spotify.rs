use base64_light::base64_encode;
use serde_json::Value;

use std::collections::VecDeque;
use std::sync::Arc;
use std::time::SystemTime;

use tokio::sync::RwLock;

use serenity::prelude::Context;

use crate::utils::structs::{BotData, Song};

struct SpotifyToken {
    token: Arc<str>,
    token_birth: SystemTime,
    token_expires_in_sec: u64,
}

impl SpotifyToken {
    fn get_valid_token(&self) -> Option<Arc<str>> {
        let sec_since_refresh = SystemTime::now()
            .duration_since(self.token_birth)
            .ok()?
            .as_secs();

        if sec_since_refresh + 10 < self.token_expires_in_sec && !self.token.is_empty() {
            Some(self.token.clone())
        } else {
            None
        }
    }
}

pub struct Spotify {
    id: Arc<str>,
    secret: Arc<str>,
    token: RwLock<SpotifyToken>,
}

impl Spotify {
    pub fn new(id: String, secret: String) -> Spotify {
        Spotify {
            id: Arc::from(id),
            secret: Arc::from(secret),
            token: RwLock::new(SpotifyToken {
                token: Arc::from(""),
                token_birth: SystemTime::now(),
                token_expires_in_sec: 0, // Token will refresh on first use
            }),
        }
    }

    async fn get_token(&self, ctx: &Context) -> Arc<str> {
        if let Some(token) = self.token.read().await.get_valid_token() {
            return token;
        }

        let mut token_info = self.token.write().await;
        if let Some(token) = token_info.get_valid_token() {
            return token;
        }

        let (new_token, expires) = Self::get_token_new(ctx, &self.id, &self.secret)
            .await
            .unwrap();

        token_info.token = new_token.clone();
        token_info.token_birth = SystemTime::now();
        token_info.token_expires_in_sec = expires;

        new_token
    }

    async fn get_token_new(ctx: &Context, id: &str, secret: &str) -> Option<(Arc<str>, u64)> {
        let auth_url = "https://accounts.spotify.com/api/token";
        let auth = base64_encode(format!("{id}:{secret}").as_str());
        let auth_code = format!("Basic {auth}");

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
            Arc::from(json["access_token"].as_str()?),
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
                let title = item["name"].as_str()?;
                let full_title = build_track_title(&item["artists"], title)?;

                album.push_back(Song::NoId {
                    title: Arc::from(full_title),
                });
            }

            if let Some(next) = json["next"].as_str() {
                next_url = next.to_string();
            } else {
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
                let title = item["track"]["name"].as_str()?;
                let full_title = build_track_title(&item["track"]["artists"], title)?;

                playlist.push_back(Song::NoId {
                    title: Arc::from(full_title),
                });
            }

            if let Some(next) = json["next"].as_str() {
                next_url = next.to_string();
            } else {
                break;
            }
        }

        Some(playlist)
    }

    pub async fn get_track(&self, ctx: &Context, id: &str) -> Option<Song> {
        let url = format!("https://api.spotify.com/v1/tracks/{}", id);

        let json = Self::https_req(self, ctx, &url).await?;

        let title = json["name"].as_str()?;
        let full_title = build_track_title(&json["artists"], title)?;

        Some(Song::NoId {
            title: Arc::from(full_title),
        })
    }

    async fn https_req(&self, ctx: &Context, url: &str) -> Option<serde_json::Value> {
        let token = Self::get_token(self, ctx).await;

        let data = ctx.data::<BotData>();

        let response = data
            .http
            .get(url)
            .header("Authorization", &format!("Bearer {token}"))
            .send()
            .await
            .ok()?;

        response.json().await.ok()
    }
}

fn build_track_title(artists: &Value, title: &str) -> Option<String> {
    let mut names: Vec<&str> = artists
        .as_array()?
        .iter()
        .filter_map(|a| a["name"].as_str())
        .collect();

    names.push(title);
    Some(names.join(" "))
}
