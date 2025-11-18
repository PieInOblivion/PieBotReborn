use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::respond::{msg_not_playing, msg_queue_stats, msg_user_not_in_voice_channel};
use crate::utils::structs::BotData;
use crate::utils::user_current_voice_and_guild::voice_and_guild;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (_, guild_id, voice_channel_id) = voice_and_guild(ctx, cmd);

    if voice_channel_id.is_none() {
        msg_user_not_in_voice_channel(ctx, cmd).await;
        return;
    }

    let (req_q, play_q) = {
        let data = ctx.data::<BotData>();
        let server_props = data.all_ser_props.get(&guild_id).unwrap().read().await;

        if server_props.playing.is_none() {
            msg_not_playing(ctx, cmd).await;
            return;
        }

        (
            server_props.request_queue.len().to_string(),
            server_props.playlist_queue.len().to_string(),
        )
    };

    msg_queue_stats(ctx, cmd, req_q, play_q).await;
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("queue").description("Shows the queue counters")
}
