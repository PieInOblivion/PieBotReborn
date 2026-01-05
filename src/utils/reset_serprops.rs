use std::collections::VecDeque;

use serenity::model::id::GuildId;
use serenity::prelude::Context;
use songbird::id::GuildId as SongbirdGuildId;

use crate::utils::structs::{AudioHandlerState, BotData};

pub async fn reset_serprops(ctx: &Context, guild_id: GuildId) -> bool {
    let data = ctx.data::<BotData>();
    let mut serprops = data.all_ser_props.get(&guild_id).unwrap().write().await;

    // Check if there's actually anything to reset
    let has_content = !serprops.request_queue.is_empty()
        || !serprops.playlist_queue.is_empty()
        || !matches!(serprops.audio_state, AudioHandlerState::Idle);

    // Clear queues and stop playback
    serprops.request_queue = VecDeque::new();
    serprops.playlist_queue = VecDeque::new();
    if let AudioHandlerState::CurrentSong { handle, .. } =
        std::mem::replace(&mut serprops.audio_state, AudioHandlerState::Idle)
    {
        let _ = handle.stop();
    }

    // Try to leave voice channel
    let manager = &data.songbird;
    let songbird_guild_id = SongbirdGuildId::from(guild_id);
    let left_vc = manager.remove(songbird_guild_id).await.is_ok();

    // Returns true if no changes were made
    !has_content && !left_vc
}
