use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType::ChannelMessageWithSource;

use crate::utils::structs::{SerProps, Song};

pub async fn msg_rps(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    bot_score: u32,
    user_score: u32,
    winner: &str,
) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0x00ffff);
    embed.title(winner);
    embed.fields(vec![("Bot", bot_score, true), ("Users", user_score, true)]);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_user_not_in_voice_channel(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0xff9900);
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
    embed.colour(0xff0000);
    embed.title(format!("No search result for: {}", query));

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_no_spotify_result(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    query: &String,
) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0xff0000);
    embed.title(format!("Spotify query failed on ID: {}", query));

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_request_queue(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    serprops: &SerProps,
    song: Song,
) {
    if serprops.playing.is_none() {
        msg_now_playing(ctx, cmd, song).await;
    } else {
        let req_q = serprops.request_queue.len().to_string();
        let play_q = serprops.playlist_queue.len().to_string();
        msg_user_queue_added(ctx, cmd, song, req_q, play_q).await;
    }
}

pub async fn msg_now_playing(ctx: &Context, cmd: &ApplicationCommandInteraction, song: Song) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0x00ffff);
    embed.title("Now Playing:");
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
    song: Song,
    req_q: String,
    play_q: String,
) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0x00ffff);
    embed.title("Added to Queue:");
    embed.fields(vec![
        (
            song.title.clone(),
            format!(
                "**https://www.youtube.com/watch?v={}**",
                song.id.clone().unwrap()
            ),
            false,
        ),
        ("User Queue Length:".to_string(), req_q, true),
        ("Playlist Queue Length:".to_string(), play_q, true),
    ]);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_list_queue_added(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    serprops: &SerProps,
    req_len_added: usize,
    play_len_added: usize,
) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0x00ffff);
    embed.title("Queue Stats");
    embed.fields(vec![
        (
            format!("User Queue: {}", serprops.request_queue.len()),
            format!("{} Added", req_len_added),
            true,
        ),
        (
            format!("Playlist Queue: {}", serprops.playlist_queue.len()),
            format!("{} Added", play_len_added),
            true,
        ),
    ]);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_queue_stats(
    ctx: &Context,
    cmd: &ApplicationCommandInteraction,
    req_len: String,
    play_len: String,
) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0x00ffff);
    embed.title("Queue Stats");
    embed.fields(vec![
        ("User Queue Length:", req_len, true),
        ("Playlist Queue Length:", play_len, true),
    ]);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_paused(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0x00ffff);
    embed.title("Paused");

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_is_paused(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0xff9900);
    embed.field("Nice.", "I'm already paused", false);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_resumed(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0x00ffff);
    embed.title("Resuming");

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_is_resumed(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0xff9900);
    embed.field("Nice.", "I'm already playing", false);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_not_playing(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0xff9900);
    embed.field("Nice.", "I'm not currently playing anything", false);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_skipped(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0x00ffff);
    embed.title("Skipped");

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_skipped_failed(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0xff9900);
    embed.field("Nice.", "There are no songs in either queues", false);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_stopped(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0x00ffff);
    embed.title("Stopped");

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_stopped_failed(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0xff9900);
    embed.field("Nice.", "You stopped NOTHING!", false);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_removed_last_song(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0x00ffff);
    embed.title("Removed last song added to user queue");

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_removed_last_song_failed(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let mut embed = CreateEmbed::default().to_owned();
    embed.colour(0xff9900);
    embed.field("Nice.", "No songs in user queue to remove", false);

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
