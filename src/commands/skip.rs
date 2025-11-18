use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::guild_and_voice_channel_id;
use crate::utils::respond::{
    msg_not_playing, msg_skipped, msg_skipped_failed, msg_user_not_in_voice_channel,
};
use crate::utils::structs::BotData;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (guild_id, voice_channel_id) = guild_and_voice_channel_id(ctx, cmd);

    if voice_channel_id.is_none() {
        msg_user_not_in_voice_channel(ctx, cmd).await;
        return;
    }

    let data = ctx.data::<BotData>();
    let server_props = data.all_ser_props.get(&guild_id).unwrap().read().await;

    if server_props.playing.is_none() {
        msg_not_playing(ctx, cmd).await;
        return;
    }

    if server_props.playlist_queue.is_empty() && server_props.request_queue.is_empty() {
        msg_skipped_failed(ctx, cmd).await;
        return;
    }

    server_props
        .playing_handle
        .as_ref()
        .unwrap()
        .stop()
        .unwrap();

    msg_skipped(ctx, cmd).await;
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("skip").description("Skips current song")
}
