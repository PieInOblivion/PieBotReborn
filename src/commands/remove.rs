use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

use crate::utils::respond::{
    msg_not_playing, msg_removed_last_song, msg_removed_last_song_failed,
    msg_user_not_in_voice_channel,
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

        let mut wait_write = allserprops.write().await;
        let serprops = wait_write.get_mut(&guild_id).unwrap();

        if serprops.playing.is_none() {
            msg_not_playing(ctx, cmd).await;
            return;
        }

        if serprops.request_queue.len() == 0 {
            msg_removed_last_song_failed(ctx, cmd).await;
            return;
        }

        serprops.request_queue.pop();
    }

    msg_removed_last_song(ctx, cmd).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("remove")
        .description("Removes the single last requested song")
}
