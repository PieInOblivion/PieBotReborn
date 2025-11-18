use serenity::all::{ChannelId, CommandInteraction, Context, GuildId};

pub fn guild_and_voice_channel_id(
    ctx: &Context,
    cmd: &CommandInteraction,
) -> (GuildId, Option<ChannelId>) {
    let guild_id = cmd.guild_id.unwrap();
    let guild = ctx.cache.guild(guild_id).unwrap();

    let voice_channel_id = guild
        .voice_states
        .get(&cmd.user.id)
        .and_then(|vs| vs.channel_id);

    (guild_id, voice_channel_id)
}
