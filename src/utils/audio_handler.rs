use serenity::client::Context;

use serenity::model::id::ChannelId;
use serenity::model::id::GuildId;

use crate::utils::structs::SerProps;

pub async fn audio_event(
    ctx: &Context,
    serprops: &mut SerProps,
    guild_id: GuildId,
    voice_channel_id: ChannelId,
) {
    // Check if playing already. If so, do nothing.
    if serprops.playing.is_some() {
        return;
    }

    if !load_next_song(serprops) {
        return;
    }

    // The song might require a youtube search if it came from spotify

    // Retrieve global songbird manager
    let manager = songbird::get(ctx).await.unwrap();

    // Check if already in the guild with the latest request
    if let Some(guild_connection) = manager.get(guild_id) {
        // Is in this guild. Check if same voice channel
        let mut call = guild_connection.lock().await;
        if call.current_channel().unwrap().0 != voice_channel_id.0 {
            // In same guild but not channel, move channels
            let _ = call.join(voice_channel_id).await;
        }
    } else {
        // TODO: deal with error joining
        // Since not in the guild yet, join
        let _ = manager.join(guild_id, voice_channel_id).await;
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(format!(
            "https://www.youtube.com/watch?v={}",
            serprops.playing.clone().unwrap().id.unwrap()
        ))
        .await
        {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:#?}", why);
                return;
            }
        };

        handler.play_source(source);
    }
}

fn load_next_song(serprops: &mut SerProps) -> bool {
    // Individual song requests take priority over playlists
    if serprops.request_queue.len() > 0 {
        serprops.playing = Some(serprops.request_queue.remove(0));
        return true;
    } else if serprops.playlist_queue.len() > 0 {
        serprops.playing = Some(serprops.playlist_queue.remove(0));
        return true;
    }
    return false;
}
