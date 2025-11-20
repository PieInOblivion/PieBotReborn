use serenity::all::{
    CommandInteraction, Context, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage, EditInteractionResponse,
};

use crate::utils::structs::Song;

// Embed Builders
pub fn create_embed_loading() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title("Loading...")
}

pub fn create_embed_user_not_in_voice_channel() -> CreateEmbed<'static> {
    CreateEmbed::default().to_owned().color(0xff9900).field(
        "I can't see you",
        "Please be in a voice channel first!",
        false,
    )
}

pub fn create_embed_no_yt_search_result() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0xff0000)
        .title("No search result")
}

pub fn create_embed_no_spotify_result() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0xff0000)
        .title("Spotify query failed")
}

pub fn create_embed_now_playing(song: &Song) -> CreateEmbed<'_> {
    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title("Now Playing:")
        .field(
            song.title(),
            format!("**https://www.youtube.com/watch?v={}**", song.id().unwrap()),
            false,
        )
}

pub fn create_embed_user_queue_added<'a>(
    song: &'a Song,
    req_q: String,
    play_q: String,
) -> CreateEmbed<'a> {
    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title("Added to Queue:")
        .fields(vec![
            (
                song.title(),
                format!("**https://www.youtube.com/watch?v={}**", song.id().unwrap()),
                false,
            ),
            ("User Queue Length:", req_q, true),
            ("Playlist Queue Length:", play_q, true),
        ])
}

pub fn create_embed_list_queue_added(
    req_len_added: usize,
    req_queue_len: usize,
    play_len_added: usize,
    playlist_queue_len: usize,
) -> CreateEmbed<'static> {
    let user_queue_title = format!("User Queue: {}", req_queue_len);
    let user_queue_value = format!("{} Added", req_len_added);
    let playlist_queue_title = format!("Playlist Queue: {}", playlist_queue_len);
    let playlist_queue_value = format!("{} Added", play_len_added);

    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title("Queue Stats")
        .fields(vec![
            (user_queue_title, user_queue_value, true),
            (playlist_queue_title, playlist_queue_value, true),
        ])
}

pub fn create_embed_rps(bot_score: u32, user_score: u32, winner: &str) -> CreateEmbed<'_> {
    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title(winner)
        .fields(vec![
            ("Bot", bot_score.to_string(), true),
            ("Users", user_score.to_string(), true),
        ])
}

pub fn create_embed_queue_stats(req_len: String, play_len: String) -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title("Queue Stats")
        .fields(vec![
            ("User Queue Length:", req_len, true),
            ("Playlist Queue Length:", play_len, true),
        ])
}

pub fn create_embed_paused() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title("Paused")
}

pub fn create_embed_is_paused() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0xff9900)
        .field("Nice.", "I'm already paused", false)
}

pub fn create_embed_resumed() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title("Resuming")
}

pub fn create_embed_is_resumed() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0xff9900)
        .field("Nice.", "I'm already playing", false)
}

pub fn create_embed_not_playing() -> CreateEmbed<'static> {
    CreateEmbed::default().to_owned().colour(0xff9900).field(
        "Nice.",
        "I'm not currently playing anything",
        false,
    )
}

pub fn create_embed_skipped() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title("Skipped")
}

pub fn create_embed_skipped_failed() -> CreateEmbed<'static> {
    CreateEmbed::default().to_owned().colour(0xff9900).field(
        "Nice.",
        "There are no songs in either queues",
        false,
    )
}

pub fn create_embed_stopped() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title("Stopped")
}

pub fn create_embed_stopped_failed() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0xff9900)
        .field("Nice.", "You stopped NOTHING!", false)
}

pub fn create_embed_removed_last_song() -> CreateEmbed<'static> {
    CreateEmbed::default()
        .to_owned()
        .colour(0x00ffff)
        .title("Removed last song added to user queue")
}

pub fn create_embed_removed_last_song_failed() -> CreateEmbed<'static> {
    CreateEmbed::default().to_owned().colour(0xff9900).field(
        "Nice.",
        "No songs in user queue to remove",
        false,
    )
}

// Send/Edit Functions
pub async fn send_embed(ctx: &Context, cmd: &CommandInteraction, embed: CreateEmbed<'_>) {
    let irm = CreateInteractionResponseMessage::new().embed(embed);
    let ir = CreateInteractionResponse::Message(irm);

    let _ = cmd.create_response(&ctx.http, ir).await;
}

pub async fn edit_embed(ctx: &Context, cmd: &CommandInteraction, embed: CreateEmbed<'_>) {
    let edit = EditInteractionResponse::new().embed(embed);

    let _ = cmd.edit_response(&ctx.http, edit).await;
}
