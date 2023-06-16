use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::prelude::TypeMapKey;

use songbird::tracks::TrackHandle;

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;

use rand::seq::SliceRandom;
use rand::thread_rng;

use tokio::sync::RwLock;

pub struct AllSerProps;

impl TypeMapKey for AllSerProps {
    type Value = HashMap<GuildId, Arc<RwLock<SerProps>>>;
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
