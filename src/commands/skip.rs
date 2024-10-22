use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::respond::{
    msg_not_playing, msg_skipped, msg_skipped_failed, msg_user_not_in_voice_channel,
};
use crate::utils::structs::AllSerProps;
use crate::utils::user_current_voice_and_guild::voice_and_guild;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (_, guild_id, voice_channel_id) = voice_and_guild(ctx, cmd);

    if voice_channel_id.is_none() {
        msg_user_not_in_voice_channel(ctx, cmd).await;
        return;
    }

    {
        let allserprops = {
            let data_read = ctx.data.read().await;
            data_read.get::<AllSerProps>().unwrap().clone()
        };
        let serprops = allserprops.get(&guild_id).unwrap().read().await;

        if serprops.playing.is_none() {
            msg_not_playing(ctx, cmd).await;
            return;
        }

        if serprops.playlist_queue.is_empty() && serprops.request_queue.is_empty() {
            msg_skipped_failed(ctx, cmd).await;
            return;
        }

        serprops.playing_handle.as_ref().unwrap().stop().unwrap();
    }

    msg_skipped(ctx, cmd).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("skip")
        .description("Skips current song")
}