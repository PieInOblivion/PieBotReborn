use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::prelude::TypeMapKey;

use songbird::tracks::TrackHandle;

use std::collections::HashMap;
use std::sync::Arc;

use rand::seq::SliceRandom;
use rand::thread_rng;

use tokio::sync::RwLock;

use rspotify::ClientCredsSpotify;

pub struct AllSerProps;

impl TypeMapKey for AllSerProps {
    type Value = HashMap<GuildId, Arc<RwLock<SerProps>>>;
}

pub struct Spotify;

impl TypeMapKey for Spotify {
    type Value = ClientCredsSpotify;
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

// TODO: Doesn't work because of lifetimes. Maybe there's something I forgot.
#[macro_export]
macro_rules! get_serprops {
    ($ctx:expr, $gid:expr) => {{
        let allserprops = {
            let data_read = $ctx.data.read().await;
            data_read.get::<AllSerProps>().unwrap().clone()
        };

        let mut wait_write = allserprops.write().await;
        let serprops = wait_write.get_mut($gid).unwrap();
        std::rc::Rc::new((serprops, wait_write, allserprops))
    }};
}
