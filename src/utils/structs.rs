use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::prelude::TypeMapKey;

use google_youtube3::YouTube;

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

pub struct YouTubeHub;

// impl TypeMapKey for YouTubeHub {
// type Value = Arc<RwLock<YouTube>>;
// }

pub struct AllSerProps;

impl TypeMapKey for AllSerProps {
    type Value = Arc<RwLock<HashMap<GuildId, SerProps>>>;
}

#[derive(Debug)]
pub struct SerProps {
    pub bot_text_channel: ChannelId,
    pub request_queue: Vec<Song>,
    pub playlist_queue: Vec<Song>,
    pub playing: Option<Song>,
}

impl SerProps {
    pub fn new(channel_id: ChannelId) -> SerProps {
        return SerProps {
            bot_text_channel: channel_id,
            request_queue: Vec::new(),
            playlist_queue: Vec::new(),
            playing: None,
        };
    }
}

#[derive(Debug)]
pub struct Song {
    pub id: String,
    pub requires_search: bool,
}

#[derive(Debug)]
pub struct SongIdentify {
    pub yt_id: Option<String>,
    pub yt_list: Option<String>,
    pub spot_track: Option<String>,
    pub spot_list: Option<String>,
    pub spot_album: Option<String>,
    pub search_needed: bool,
}
