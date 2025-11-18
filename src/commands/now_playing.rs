use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::guild_and_voice_channel_id;
use crate::utils::respond::{msg_not_playing, msg_now_playing, msg_user_not_in_voice_channel};
use crate::utils::structs::BotData;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (guild_id, voice_channel_id) = guild_and_voice_channel_id(ctx, cmd);

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
