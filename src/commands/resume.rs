use serenity::all::{CommandInteraction, Context, CreateCommand};

use songbird::tracks::PlayMode;

use crate::utils::guild_and_voice_channel_id;
use crate::utils::respond::{
    create_embed_is_resumed, create_embed_not_playing, create_embed_resumed,
    create_embed_user_not_in_voice_channel, send_embed,
};
use crate::utils::structs::BotData;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (guild_id, voice_channel_id) = guild_and_voice_channel_id(ctx, cmd);

    if voice_channel_id.is_none() {
        send_embed(ctx, cmd, create_embed_user_not_in_voice_channel()).await;
        return;
    }

    let data = ctx.data::<BotData>();
    let server_props = data.all_ser_props.get(&guild_id).unwrap().read().await;

    if server_props.playing.is_none() {
        send_embed(ctx, cmd, create_embed_not_playing()).await;
        return;
    }

    let handle = server_props.playing_handle.as_ref().unwrap();

    if handle.get_info().await.unwrap().playing == PlayMode::Play {
        send_embed(ctx, cmd, create_embed_is_resumed()).await;
        return;
    }

    handle.play().unwrap();

    send_embed(ctx, cmd, create_embed_resumed()).await;
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("resume").description("Resume current song")
}
