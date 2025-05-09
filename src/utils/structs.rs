use serenity::model::id::GuildId;
use serenity::prelude::TypeMapKey;

use songbird::tracks::TrackHandle;

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use rand::seq::SliceRandom;

use tokio::sync::RwLock;

pub struct AllSerProps;

impl TypeMapKey for AllSerProps {
    type Value = HashMap<GuildId, Arc<RwLock<SerProps>>>;
}

#[derive(Clone)]
pub struct SerProps {
    pub request_queue: VecDeque<Song>,
    pub playlist_queue: VecDeque<Song>,
    pub playing: Option<Song>,
    pub playing_handle: Option<TrackHandle>,
}

impl SerProps {
    pub fn new() -> SerProps {
        SerProps {
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
    pub id: Option<String>,
    pub title: String,
}

pub struct SongFilterResult {
    pub yt_id: Option<String>,
    pub yt_list: Option<String>,
    pub spot_track: Option<String>,
    pub spot_list: Option<String>,
    pub spot_album: Option<String>,
    pub search_needed: bool,
}
