use serenity::all::{CommandInteraction, Context, CreateEmbed, CreateInteractionResponse, CreateInteractionResponseMessage};

use crate::utils::structs::{SerProps, Song};

pub async fn msg_rps(
    ctx: &Context,
    cmd: &CommandInteraction,
    bot_score: u32,
    user_score: u32,
    winner: &str,
) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0x00ffff)
    .title(winner)
    .fields(vec![("Bot", bot_score.to_string(), true), ("Users", user_score.to_string(), true)]);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_user_not_in_voice_channel(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .color(0xff9900)
    .field(
        "I can't see you",
        "Please be in a voice channel first!",
        false,
    );

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_no_yt_search_result(
    ctx: &Context,
    cmd: &CommandInteraction,
    query: &String,
) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0xff0000)
    .title(format!("No search result for: {}", query));

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_no_spotify_result(
    ctx: &Context,
    cmd: &CommandInteraction,
    query: &String,
) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0xff0000)
    .title(format!("Spotify query failed on ID: {}", query));

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_request_queue(
    ctx: &Context,
    cmd: &CommandInteraction,
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

pub async fn msg_now_playing(ctx: &Context, cmd: &CommandInteraction, song: Song) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0x00ffff)
    .title("Now Playing:")
    .field(
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
    cmd: &CommandInteraction,
    song: Song,
    req_q: String,
    play_q: String,
) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0x00ffff)
    .title("Added to Queue:")
    .fields(vec![
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
    cmd: &CommandInteraction,
    serprops: &SerProps,
    req_len_added: usize,
    play_len_added: usize,
) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0x00ffff)
    .title("Queue Stats")
    .fields(vec![
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
    cmd: &CommandInteraction,
    req_len: String,
    play_len: String,
) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0x00ffff)
    .title("Queue Stats")
    .fields(vec![
        ("User Queue Length:", req_len, true),
        ("Playlist Queue Length:", play_len, true),
    ]);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_paused(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0x00ffff)
    .title("Paused");

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_is_paused(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0xff9900)
    .field("Nice.", "I'm already paused", false);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_resumed(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0x00ffff)
    .title("Resuming");

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_is_resumed(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0xff9900)
    .field("Nice.", "I'm already playing", false);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_not_playing(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0xff9900)
    .field("Nice.", "I'm not currently playing anything", false);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_skipped(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0x00ffff)
    .title("Skipped");

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_skipped_failed(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0xff9900)
    .field("Nice.", "There are no songs in either queues", false);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_stopped(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0x00ffff)
    .title("Stopped");

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_stopped_failed(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0xff9900)
    .field("Nice.", "You stopped NOTHING!", false);

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_removed_last_song(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0x00ffff)
    .title("Removed last song added to user queue");

    send_embed(ctx, cmd, embed).await;
}

pub async fn msg_removed_last_song_failed(ctx: &Context, cmd: &CommandInteraction) {
    let embed = CreateEmbed::default().to_owned()
    .colour(0xff9900)
    .field("Nice.", "No songs in user queue to remove", false);

    send_embed(ctx, cmd, embed).await;
}

async fn send_embed(ctx: &Context, cmd: &CommandInteraction, embed: CreateEmbed) {
    let irm = CreateInteractionResponseMessage::new().embed(embed);
    let ir = CreateInteractionResponse::Message(irm);

    cmd.create_response(&ctx.http, ir).await.unwrap();
}