use serenity::all::{CommandInteraction, Context, CreateCommand};

use songbird::tracks::PlayMode;

use crate::utils::respond::{
    msg_is_paused, msg_not_playing, msg_paused, msg_user_not_in_voice_channel,
};
use crate::utils::structs::BotData;
use crate::utils::user_current_voice_and_guild::voice_and_guild;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (_, guild_id, voice_channel_id) = voice_and_guild(ctx, cmd);

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

    let handle = server_props.playing_handle.as_ref().unwrap();

    if handle.get_info().await.unwrap().playing == PlayMode::Pause {
        msg_is_paused(ctx, cmd).await;
        return;
    }

    handle.pause().unwrap();

    msg_paused(ctx, cmd).await;
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("pause").description("Pauses current song")
}
