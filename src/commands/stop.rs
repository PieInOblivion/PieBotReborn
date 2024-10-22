use serenity::all::{CommandInteraction, Context, CreateCommand};

use crate::utils::reset_serprops::reset_serprops;
use crate::utils::respond::{msg_stopped, msg_stopped_failed};
use crate::utils::user_current_voice_and_guild::voice_and_guild;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (_, guild_id, _) = voice_and_guild(ctx, cmd);

    let was_not_changed = reset_serprops(ctx, guild_id).await;

    if was_not_changed {
        msg_stopped_failed(ctx, cmd).await;
    } else {
        msg_stopped(ctx, cmd).await;
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stop")
        .description("Stops current song, deletes queues, leaves voice channel")
}