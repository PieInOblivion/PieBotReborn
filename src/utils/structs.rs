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
    pub youtube_key: String,
}

pub struct ServerProps {
    pub request_queue: VecDeque<Song>,
    pub playlist_queue: VecDeque<Song>,
    pub audio_state: AudioHandlerState,
}

impl ServerProps {
    pub fn new() -> ServerProps {
        ServerProps {
            request_queue: VecDeque::new(),
            playlist_queue: VecDeque::new(),
            audio_state: AudioHandlerState::Idle,
        }
    }

    pub fn playlist_queue_shuffle(&mut self) {
        self.playlist_queue
            .make_contiguous()
            .shuffle(&mut rand::rng());
    }
}

#[derive(Clone)]
pub enum Song {
    NoId { title: Arc<str> },
    WithId { id: Arc<str>, title: Arc<str> },
}

impl Song {
    pub fn title(&self) -> &str {
        match self {
            Song::NoId { title } | Song::WithId { title, .. } => title,
        }
    }

    pub fn id(&self) -> Option<&str> {
        match self {
            Song::NoId { .. } => None,
            Song::WithId { id, .. } => Some(id),
        }
    }
}

pub enum AudioHandlerState {
    Idle,
    BetweenSongs { past_song: Song },
    CurrentSong { song: Song, handle: TrackHandle },
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
