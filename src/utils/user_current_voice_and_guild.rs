use serenity::all::{Guild, GuildId, ChannelId, Context, CommandInteraction};

pub fn voice_and_guild(
    ctx: &Context,
    cmd: &CommandInteraction,
) -> (Guild, GuildId, Option<ChannelId>) {
    let guild_id = cmd.guild_id.unwrap();
    let guild = ctx.cache.guild(guild_id).unwrap();

    let voice_channel_id = guild
        .voice_states
        .get(&cmd.user.id)
        .and_then(|vs| vs.channel_id);

    (guild.clone(), guild_id, voice_channel_id)
}
