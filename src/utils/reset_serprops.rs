use serenity::client::Context;
use serenity::model::id::GuildId;

use crate::utils::structs::AllSerProps;

pub async fn reset_serprops(ctx: &Context, guild_id: GuildId) -> bool {
    let allserprops = {
        let data_read = ctx.data.read().await;
        data_read.get::<AllSerProps>().unwrap().clone()
    };

    let mut wait_write = allserprops.write().await;
    let serprops = wait_write.get_mut(&guild_id).unwrap();

    let old_serprops = serprops.clone();
    let mut left_vc = false;

    serprops.request_queue = Vec::new();
    serprops.playlist_queue = Vec::new();
    serprops.playing = None;

    if serprops.playing_handle.is_some() {
        let _ = serprops.playing_handle.as_ref().unwrap().stop();
        serprops.playing_handle = None;
    }

    let manager = songbird::get(ctx).await.unwrap();
    if manager.remove(guild_id).await.is_ok() {
        left_vc = true;
    }

    // Returns true if no changes were made
    old_serprops.playing == serprops.playing
        && old_serprops.playlist_queue == serprops.playlist_queue
        && old_serprops.request_queue == serprops.request_queue
        && old_serprops.playing_handle.is_none()
        && serprops.playing_handle.is_none()
        && !left_vc
}
