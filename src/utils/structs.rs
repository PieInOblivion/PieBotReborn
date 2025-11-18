use serenity::model::id::GuildId;

use songbird::Songbird;
use songbird::tracks::TrackHandle;

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use rand::seq::SliceRandom;

use tokio::sync::RwLock;

use crate::utils::spotify::Spotify;
use reqwest::Client as HttpClient;

pub struct BotData {
    pub all_ser_props: HashMap<GuildId, RwLock<ServerProps>>,
    pub spotify: Spotify,
    pub http: HttpClient,
    pub songbird: Arc<Songbird>,
}

pub struct ServerProps {
    pub request_queue: VecDeque<Song>,
    pub playlist_queue: VecDeque<Song>,
    pub playing: Option<Song>,
    pub playing_handle: Option<TrackHandle>,
}

impl ServerProps {
    pub fn new() -> ServerProps {
        ServerProps {
            request_queue: VecDeque::new(),
            playlist_queue: VecDeque::new(),
            playing: None,
            playing_handle: None,
        }
    }

    pub fn playlist_queue_shuffle(&mut self) {
        self.playlist_queue
            .make_contiguous()
            .shuffle(&mut rand::rng());
    }
}

#[derive(Clone, PartialEq)]
pub struct Song {
    pub id: Option<Arc<str>>,
    pub title: Arc<str>,
}

pub enum PlayRequest {
    Search(String),
    YouTubeVideo(String),
    YouTubePlaylist(String),
    YouTubeVideoAndPlaylist { video: String, playlist: String },
    SpotifyTrack(String),
    SpotifyPlaylist(String),
    SpotifyAlbum(String),
}
