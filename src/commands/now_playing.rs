use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::respond::{msg_not_playing, msg_now_playing, msg_user_not_in_voice_channel};
use crate::utils::structs::AllSerProps;
use crate::utils::user_current_voice_and_guild::voice_and_guild;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (_, guild_id, voice_channel_id) = voice_and_guild(ctx, cmd);

    if voice_channel_id.is_none() {
        msg_user_not_in_voice_channel(ctx, cmd).await;
        return;
    }

    let song = {
        let allserprops = {
            let data_read = ctx.data.read().await;
            data_read.get::<AllSerProps>().unwrap().clone()
        };
        let serprops = allserprops.get(&guild_id).unwrap().read().await;

        if serprops.playing.is_none() {
            msg_not_playing(ctx, cmd).await;
            return;
        }

        serprops.playing.clone().unwrap()
    };

    msg_now_playing(ctx, cmd, song).await;
}

pub fn register() -> CreateCommand {
    CreateCommand::new("np")
        .description("Shows the song currently playing")
}
