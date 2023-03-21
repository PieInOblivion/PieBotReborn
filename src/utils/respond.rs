use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource;

pub async fn msg_ping(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.title("Hey, I'm alive!");
    embed.colour((255, 255, 255));

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_rps(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    bot_score: u32,
    user_score: u32,
    winner: &str,
) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.title(winner);
    embed.colour((0, 255, 255));
    embed.fields(vec![("Bot", bot_score, true), ("Users", user_score, true)]);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_user_not_in_voice_channel(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour((255, 153, 0));
    embed.field(
        "I can't see you",
        "Please be in a voice channel first!",
        false,
    );

    send_embed(ctx, cmd, embed).await;
}

async fn send_embed(ctx: &Context, cmd: &ApplicationCommandInteraction, embed: CreateEmbed) {
    cmd.create_interaction_response(&ctx.http, |response| {
        response
            .kind(ChannelMessageWithSource)
            .interaction_response_data(|msg| msg.set_embed(embed))
    })
    .await
    .unwrap();
}
