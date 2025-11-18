use serenity::model::id::GuildId;
use serenity::prelude::Context;
use songbird::id::GuildId as SongbirdGuildId;

use crate::utils::structs::BotData;

pub async fn reset_serprops(ctx: &Context, guild_id: GuildId) -> bool {
    let data = ctx.data::<BotData>();
    let mut serprops = data.all_ser_props.get(&guild_id).unwrap().write().await;

    // Check if there's actually anything to reset
    let has_content = !serprops.request_queue.is_empty()
        || !serprops.playlist_queue.is_empty()
        || serprops.playing.is_some()
        || serprops.playing_handle.is_some();

    // Clear queues and stop playback
    serprops.request_queue.clear();
    serprops.playlist_queue.clear();
    serprops.playing = None;

    if let Some(handle) = serprops.playing_handle.take() {
        let _ = handle.stop();
    }

    // Try to leave voice channel
    let manager = &data.songbird;
    let songbird_guild_id = SongbirdGuildId::from(guild_id);
    let left_vc = manager.remove(songbird_guild_id).await.is_ok();

    // Returns true if no changes were made
    !has_content && !left_vc
}
