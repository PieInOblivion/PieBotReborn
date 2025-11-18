use serenity::async_trait;
use serenity::model::id::{ChannelId, GuildId};
use serenity::prelude::Context;

use songbird::id::{ChannelId as SongbirdChannelId, GuildId as SongbirdGuildId};
use songbird::input::YoutubeDl;
use songbird::{Event, EventContext, EventHandler, TrackEvent};

use crate::utils::reset_serprops::reset_serprops;
use crate::utils::structs::{BotData, ServerProps};
use crate::utils::youtube::yt_search;

pub async fn audio_event(ctx: &Context, guild_id: GuildId, voice_channel_id: ChannelId) {
    // Check if playing already. If so, do nothing.
    let song = {
        let data = ctx.data::<BotData>();
        let mut serprops = data.all_ser_props.get(&guild_id).unwrap().write().await;

        if serprops.playing.is_some() {
            return;
        }

        if !load_next_song(ctx, &mut serprops).await {
            drop(serprops);
            reset_serprops(ctx, guild_id).await;
            return;
        }

        serprops.playing.clone().unwrap()
    };

    let source_url = format!(
        "https://www.youtube.com/watch?v={}",
        song.id.as_ref().unwrap()
    );

    let data = ctx.data::<BotData>();
    let source = YoutubeDl::new(data.http.clone(), source_url);

    // Get songbird manager from BotData (already Arc-wrapped)
    let manager = &data.songbird;

    // Convert serenity IDs to songbird IDs (direct conversion supported in serenity-next)
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

async fn load_next_song(ctx: &Context, serprops: &mut ServerProps) -> bool {
    loop {
        serprops.playing = serprops
            .request_queue
            .pop_front()
            .or_else(|| serprops.playlist_queue.pop_front());

        if let Some(playing) = &serprops.playing {
            if playing.id.is_some() {
                return true;
            } else if let Some(new_song_data) = yt_search(ctx, &playing.title).await {
                serprops.playing = Some(new_song_data);
                return true;
            }
        } else {
            return false;
        }
    }
}
