use tokio::sync::RwLock;

use serenity::async_trait;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::Context;

use songbird::id::{ChannelId as SongbirdChannelId, GuildId as SongbirdGuildId};
use songbird::input::YoutubeDl;
use songbird::{Event, EventContext, EventHandler, TrackEvent};

use crate::utils::reset_serprops::reset_serprops;
use crate::utils::structs::{BotData, ServerProps, Song};
use crate::utils::youtube::yt_search;

pub async fn audio_event(ctx: &Context, guild_id: GuildId, voice_channel_id: ChannelId) {
    let data = ctx.data::<BotData>();

    // Check if playing already. If so, do nothing.
    {
        let serprops = data.all_ser_props.get(&guild_id).unwrap().read().await;
        if serprops.playing.is_some() {
            return;
        }
    }

    let serprops_lock = data.all_ser_props.get(&guild_id).unwrap();
    let song = match load_next_song(ctx, serprops_lock).await {
        Some(song) => {
            let mut serprops = serprops_lock.write().await;
            serprops.playing = Some(song.clone());
            song
        }
        None => {
            reset_serprops(ctx, guild_id).await;
            return;
        }
    };

    let source_url = format!("https://www.youtube.com/watch?v={}", song.id().unwrap());
    let source = YoutubeDl::new(data.http.clone(), source_url);

    let manager = &data.songbird;

    let songbird_guild_id = SongbirdGuildId::from(guild_id);
    let songbird_channel_id = SongbirdChannelId::from(voice_channel_id);

    let call = {
        if let Some(call) = manager.get(songbird_guild_id) {
            call
        } else {
            let call = manager
                .join(songbird_guild_id, songbird_channel_id)
                .await
                .unwrap();
            let mut call_lock = call.lock().await;

            call_lock.add_global_event(
                Event::Track(TrackEvent::End),
                TrackEndNotifier {
                    guild_id,
                    voice_channel_id,
                    ctx: ctx.clone(),
                },
            );

            drop(call_lock);

            call
        }
    };

    let mut call_lock = call.lock().await;

    let mut serprops = data.all_ser_props.get(&guild_id).unwrap().write().await;
    serprops.playing_handle = Some(call_lock.play_input(source.clone().into()));
}

struct TrackEndNotifier {
    guild_id: GuildId,
    voice_channel_id: ChannelId,
    ctx: Context,
}

#[async_trait]
impl EventHandler for TrackEndNotifier {
    async fn act(&self, _: &EventContext<'_>) -> Option<Event> {
        {
            let data = self.ctx.data::<BotData>();
            let mut serprops = data
                .all_ser_props
                .get(&self.guild_id)
                .unwrap()
                .write()
                .await;

            serprops.playing = None;
            serprops.playing_handle = None;
        }

        audio_event(&self.ctx, self.guild_id, self.voice_channel_id).await;

        None
    }
}

async fn load_next_song(ctx: &Context, serprops_lock: &RwLock<ServerProps>) -> Option<Song> {
    loop {
        let option_song = {
            let mut serprops = serprops_lock.write().await;
            serprops
                .request_queue
                .pop_front()
                .or_else(|| serprops.playlist_queue.pop_front())
        };

        match option_song {
            Some(Song::WithId { .. }) => return option_song,
            Some(Song::NoId { title }) => {
                if let Some(searched) = yt_search(ctx, &title).await {
                    return Some(searched);
                }
            }
            None => return None,
        }
    }
}
