use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use crate::utils::respond::{msg_not_playing, msg_queue_stats, msg_user_not_in_voice_channel};
use crate::utils::structs::AllSerProps;
use crate::utils::user_current_voice_and_guild::voice_and_guild;

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let (_, guild_id, voice_channel_id) = voice_and_guild(ctx, cmd);

    if voice_channel_id == None {
        msg_user_not_in_voice_channel(ctx, cmd).await;
        return;
    }

    let (req_q, play_q) = {
        let allserprops = {
            let data_read = ctx.data.read().await;
            data_read.get::<AllSerProps>().unwrap().clone()
        };

        let wait_read = allserprops.read().await;
        let serprops = wait_read.get(&guild_id).unwrap();

        if serprops.playing.is_none() {
            msg_not_playing(ctx, cmd).await;
            return;
        }

        (
            serprops.request_queue.len().to_string(),
            serprops.playlist_queue.len().to_string(),
        )
    };

    msg_queue_stats(ctx, cmd, req_q, play_q).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("queue")
        .description("Shows the queue counters")
}
