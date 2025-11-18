use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::respond::{msg_not_playing, msg_now_playing, msg_user_not_in_voice_channel};
use crate::utils::structs::BotData;
use crate::utils::user_current_voice_and_guild::voice_and_guild;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (_, guild_id, voice_channel_id) = voice_and_guild(ctx, cmd);

    if voice_channel_id.is_none() {
        msg_user_not_in_voice_channel(ctx, cmd).await;
        return;
    }

    let song = {
        let data = ctx.data::<BotData>();
        let server_props = data.all_ser_props.get(&guild_id).unwrap().read().await;

        if server_props.playing.is_none() {
            msg_not_playing(ctx, cmd).await;
            return;
        }

        server_props.playing.clone().unwrap()
    };

    msg_now_playing(ctx, cmd, song).await;
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("np").description("Shows the song currently playing")
}
