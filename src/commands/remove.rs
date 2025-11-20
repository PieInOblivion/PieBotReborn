use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::guild_and_voice_channel_id;
use crate::utils::respond::{
    create_embed_not_playing, create_embed_removed_last_song,
    create_embed_removed_last_song_failed, create_embed_user_not_in_voice_channel, send_embed,
};
use crate::utils::structs::BotData;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (guild_id, Some(_voice_channel_id)) = guild_and_voice_channel_id(ctx, cmd) else {
        send_embed(ctx, cmd, create_embed_user_not_in_voice_channel()).await;
        return;
    };

    let data = ctx.data::<BotData>();
    let mut server_props = data.all_ser_props.get(&guild_id).unwrap().write().await;

    if server_props.playing.is_none() {
        send_embed(ctx, cmd, create_embed_not_playing()).await;
        return;
    }

    if server_props.request_queue.pop_back().is_none() {
        send_embed(ctx, cmd, create_embed_removed_last_song_failed()).await;
        return;
    }

    send_embed(ctx, cmd, create_embed_removed_last_song()).await;
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("remove").description("Removes the single last requested song")
}
