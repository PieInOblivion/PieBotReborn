use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::guild_and_voice_channel_id;
use crate::utils::respond::{
    create_embed_not_playing, create_embed_queue_stats, create_embed_user_not_in_voice_channel,
    send_embed,
};
use crate::utils::structs::{AudioHandlerState, BotData};

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (guild_id, Some(_voice_channel_id)) = guild_and_voice_channel_id(ctx, cmd) else {
        send_embed(ctx, cmd, create_embed_user_not_in_voice_channel()).await;
        return;
    };

    let (req_q, play_q) = {
        let data = ctx.data::<BotData>();
        let server_props = data.all_ser_props.get(&guild_id).unwrap().read().await;

        if matches!(server_props.audio_state, AudioHandlerState::Idle) {
            send_embed(ctx, cmd, create_embed_not_playing()).await;
            return;
        }

        (
            server_props.request_queue.len().to_string(),
            server_props.playlist_queue.len().to_string(),
        )
    };

    send_embed(ctx, cmd, create_embed_queue_stats(req_q, play_q)).await;
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("queue").description("Shows the queue counters")
}
