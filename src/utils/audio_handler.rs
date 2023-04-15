use serenity::async_trait;
use serenity::client::Context;
use serenity::model::id::{ChannelId, GuildId};

use songbird::{Event, EventContext, EventHandler};

use crate::utils::query_youtube::yt_search;
use crate::utils::reset_serprops::reset_serprops;
use crate::utils::structs::{AllSerProps, SerProps};

pub async fn audio_event(ctx: &Context, guild_id: GuildId, voice_channel_id: ChannelId) {
    // Check if playing already. If so, do nothing.
    let song = {
        let mut allserprops = {
            let data_read = ctx.data.read().await;
            data_read.get::<AllSerProps>().unwrap().clone()
        };
        let mut serprops = allserprops.get_mut(&guild_id).unwrap().write().await;

        if serprops.playing.is_some() {
            return;
        }

        if !load_next_song(&mut serprops).await {
            drop(serprops);
            reset_serprops(ctx, guild_id).await;
            return;
        }

        serprops.playing.clone().unwrap()
    };

    let source = match songbird::ytdl(format!(
        "https://www.youtube.com/watch?v={}",
        song.id.as_ref().unwrap()
    ))
    .await
    {
        Ok(source) => source,
        Err(err) => {
            println!("Download failed: {:#?}\n{:#?}", song, err);
            return;
        }
    };

    let manager = songbird::get(ctx).await.unwrap();

    let call = {
        if let Some(call) = manager.get(guild_id) {
            call
        } else {
            let call = manager.join(guild_id, voice_channel_id).await.0;
            let mut call_lock = call.lock().await;

            call_lock.add_global_event(
                songbird::Event::Track(songbird::TrackEvent::End),
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

    {
        let mut allserprops = {
            let data_read = ctx.data.read().await;
            data_read.get::<AllSerProps>().unwrap().clone()
        };
        let mut serprops = allserprops.get_mut(&guild_id).unwrap().write().await;
        serprops.playing_handle = Some(call_lock.play_source(source));
    }

    drop(call_lock);
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
            let mut allserprops = {
                let data_read = self.ctx.data.read().await;
                data_read.get::<AllSerProps>().unwrap().clone()
            };

            let mut serprops = allserprops.get_mut(&self.guild_id).unwrap().write().await;

            serprops.playing = None;
            serprops.playing_handle = None;
        }

        audio_event(&self.ctx, self.guild_id, self.voice_channel_id).await;

        None
    }
}

async fn load_next_song(serprops: &mut SerProps) -> bool {
    // Individual song requests take priority over playlists
    if serprops.request_queue.len() > 0 {
        serprops.playing = Some(serprops.request_queue.remove(0));
    } else if serprops.playlist_queue.len() > 0 {
        serprops.playing = Some(serprops.playlist_queue.remove(0));
    }

    // Song has no youtube ID, requires query
    if serprops.playing.as_ref().unwrap().id.is_none() {
        if let Some(new_song) = yt_search(&serprops.playing.as_ref().unwrap().title).await {
            serprops.playing = Some(new_song);
            return true;
        }
    } else {
        return true;
    }

    return false;
}
