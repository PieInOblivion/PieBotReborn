use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource;

use crate::utils::structs::{SerProps, Song};

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

pub async fn msg_no_yt_search_result(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    query: &String,
) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.title(format!("Search failed: {}", query));
    embed.colour((255, 0, 0));

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_request_queue(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    serprops: &SerProps,
    song: &Song,
) {
    if serprops.playing.is_none() {
        msg_now_playing(ctx, cmd, song).await;
    } else {
        msg_user_queue_added(ctx, cmd, serprops, song).await;
    }
}

async fn msg_now_playing(ctx: &Context, cmd: &ApplicationCommandInteraction, song: &Song) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.title("Now Playing:");
    embed.colour((0, 255, 255));
    embed.field(
        song.title.clone(),
        format!(
            "**https://www.youtube.com/watch?v={}**",
            song.id.clone().unwrap()
        ),
        false,
    );

    send_embed(ctx, cmd, embed).await;
}

async fn msg_user_queue_added(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    serprops: &SerProps,
    song: &Song,
) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.title("Added to Queue:");
    embed.colour((0, 255, 255));
    embed.fields(vec![
        (
            song.title.clone(),
            format!(
                "**https://www.youtube.com/watch?v={}**",
                song.id.clone().unwrap()
            ),
            false,
        ),
        (
            "User Queue Length:".to_string(),
            serprops.request_queue.len().to_string(),
            true,
        ),
        (
            "Playlist Queue Length:".to_string(),
            serprops.playlist_queue.len().to_string(),
            true,
        ),
    ]);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_list_queue_added(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    serprops: &SerProps,
    len: usize,
) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.title("Queue Stats");
    embed.colour((0, 255, 255));
    embed.fields(vec![
        (
            format!("User Queue: {}", serprops.request_queue.len()),
            "0 Added".to_string(),
            true,
        ),
        (
            format!("Playlist Queue: {}", serprops.playlist_queue.len()),
            format!("{} Added", len),
            true,
        ),
    ]);

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
