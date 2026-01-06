use serenity::async_trait;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::Context;

use songbird::id::{ChannelId as SongbirdChannelId, GuildId as SongbirdGuildId};
use songbird::input::YoutubeDl;
use songbird::{Event, EventContext, EventHandler, TrackEvent};

use crate::utils::reset_serprops::reset_serprops;
use crate::utils::structs::{AudioHandlerState, BotData, ServerProps, Song};
use crate::utils::youtube::yt_search;

pub async fn audio_event(
    ctx: &Context,
    guild_id: GuildId,
    voice_channel_id: ChannelId,
    goto_next_song: bool,
) {
    let data = ctx.data::<BotData>();

    let serprops_lock = data.all_ser_props.get(&guild_id).unwrap();

    // Check if playing already. If so, do nothing.
    if !goto_next_song
        && !matches!(
            serprops_lock.read().await.audio_state,
            AudioHandlerState::Idle
        )
    {
        return;
    }

    let song = match load_next_song(ctx, serprops_lock).await {
        Some(song) => song,
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

    let call = if let Some(call) = manager.get(songbird_guild_id) {
        call
    } else {
        let call = manager
            .join(songbird_guild_id, songbird_channel_id)
            .await
            .unwrap();

        call.lock().await.add_global_event(
            Event::Track(TrackEvent::End),
            TrackEndNotifier {
                guild_id,
                voice_channel_id,
                ctx: ctx.clone(),
            },
        );

        call
    };

    let mut call_lock = call.lock().await;
    let mut serprops = serprops_lock.write().await;
    let handle = call_lock.play_input(source.into());
    serprops.audio_state = AudioHandlerState::Playing {
        song: song.clone(),
        handle,
    };
}

struct TrackEndNotifier {
    guild_id: GuildId,
    voice_channel_id: ChannelId,
    ctx: Context,
}

#[async_trait]
impl EventHandler for TrackEndNotifier {
    async fn act(&self, _: &EventContext<'_>) -> Option<Event> {
        audio_event(&self.ctx, self.guild_id, self.voice_channel_id, true).await;

        None
    }
}

async fn load_next_song(
    ctx: &Context,
    serprops_lock: &tokio::sync::RwLock<ServerProps>,
) -> Option<Song> {
    loop {
        let next_song = {
            let mut serprops = serprops_lock.write().await;

            serprops
                .request_queue
                .pop_front()
                .or_else(|| serprops.playlist_queue.pop_front())
        };

        match next_song {
            Some(Song::WithId { .. }) => return next_song,
            Some(Song::NoId { title }) => {
                if let Some(searched) = yt_search(ctx, &title).await {
                    return Some(searched);
                }
            }
            None => return None,
        }
    }
}
