use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use crate::utils::respond::{
    msg_not_playing, msg_skipped, msg_skipped_failed, msg_user_not_in_voice_channel,
};
use crate::utils::structs::AllSerProps;
use crate::utils::user_current_voice_and_guild::voice_and_guild;

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let (_, guild_id, voice_channel_id) = voice_and_guild(ctx, cmd);

    if voice_channel_id == None {
        msg_user_not_in_voice_channel(ctx, cmd).await;
        return;
    }

    {
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

        if serprops.playlist_queue.len() == 0 && serprops.request_queue.len() == 0 {
            msg_skipped_failed(ctx, cmd).await;
            return;
        }

        serprops.playing_handle.as_ref().unwrap().stop().unwrap();
    }

    msg_skipped(ctx, cmd).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("skip").description("Skips current song")
}
