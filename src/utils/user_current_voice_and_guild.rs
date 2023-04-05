use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::guild::Guild;
use serenity::model::id::{ChannelId, GuildId};

pub fn voice_and_guild(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
) -> (Guild, GuildId, Option<ChannelId>) {
    let guild_id = cmd.guild_id.unwrap();
    let guild = ctx.cache.guild(guild_id).unwrap();

    let voice_channel_id = guild
        .voice_states
        .get(&cmd.user.id)
        .and_then(|vs| vs.channel_id);

    (guild, guild_id, voice_channel_id)
}
