use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::guild_and_voice_channel_id;
use crate::utils::reset_serprops::reset_serprops;
use crate::utils::respond::{msg_stopped, msg_stopped_failed};

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (guild_id, _) = guild_and_voice_channel_id(ctx, cmd);

    let was_not_changed = reset_serprops(ctx, guild_id).await;

    if was_not_changed {
        msg_stopped_failed(ctx, cmd).await;
    } else {
        msg_stopped(ctx, cmd).await;
    }
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("stop")
        .description("Stops current song, deletes queues, leaves voice channel")
}
