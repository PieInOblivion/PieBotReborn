use serenity::all::{CommandInteraction, Context, CreateCommand};

use songbird::tracks::PlayMode;

use crate::utils::guild_and_voice_channel_id;
use crate::utils::respond::{
    create_embed_is_paused, create_embed_not_playing, create_embed_paused,
    create_embed_user_not_in_voice_channel, send_embed,
};
use crate::utils::structs::{AudioHandlerState, BotData};

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (guild_id, Some(_voice_channel_id)) = guild_and_voice_channel_id(ctx, cmd) else {
        send_embed(ctx, cmd, create_embed_user_not_in_voice_channel()).await;
        return;
    };

    let data = ctx.data::<BotData>();
    let server_props = data.all_ser_props.get(&guild_id).unwrap().read().await;

    let handle = match &server_props.audio_state {
        AudioHandlerState::CurrentSong { handle, .. } => handle,
        _ => {
            send_embed(ctx, cmd, create_embed_not_playing()).await;
            return;
        }
    };

    if handle.get_info().await.unwrap().playing == PlayMode::Pause {
        send_embed(ctx, cmd, create_embed_is_paused()).await;
        return;
    }

    handle.pause().unwrap();

    send_embed(ctx, cmd, create_embed_paused()).await;
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("pause").description("Pauses current song")
}
