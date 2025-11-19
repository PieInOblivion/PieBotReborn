use std::collections::VecDeque;

use crate::utils::audio_handler::audio_event;
use crate::utils::guild_and_voice_channel_id;
use crate::utils::identify_source::parse_source;
use crate::utils::interaction::arg_to_str;
use crate::utils::respond::{
    create_embed_list_queue_added, create_embed_loading, create_embed_no_spotify_result,
    create_embed_no_yt_search_result, create_embed_now_playing,
    create_embed_user_not_in_voice_channel, create_embed_user_queue_added, edit_embed, send_embed,
};
use crate::utils::structs::{BotData, PlayRequest, Song};
use crate::utils::youtube::{yt_id_to_name, yt_list_id_to_vec, yt_search};

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
};
use serenity::model::id::GuildId;

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    send_embed(ctx, cmd, create_embed_loading()).await;

    let (guild_id, voice_channel_id) = guild_and_voice_channel_id(ctx, cmd);

    if voice_channel_id.is_none() {
        edit_embed(ctx, cmd, create_embed_user_not_in_voice_channel()).await;
        return;
    }

    let user_query = arg_to_str(cmd);
    let request = parse_source(user_query);
    let data = ctx.data::<BotData>();

    match request {
        PlayRequest::Search(query) => {
            if let Some(song) = yt_search(ctx, &query).await {
                add_single_song(ctx, cmd, guild_id, &data, song).await;
            } else {
                edit_embed(ctx, cmd, create_embed_no_yt_search_result()).await;
            }
        }

        PlayRequest::YouTubeVideo(id) => {
            if let Some(song) = yt_id_to_name(ctx, &id).await {
                add_single_song(ctx, cmd, guild_id, &data, song).await;
            } else {
                edit_embed(ctx, cmd, create_embed_no_yt_search_result()).await;
            }
        }

        PlayRequest::YouTubePlaylist(id) => {
            if let Some(list) = yt_list_id_to_vec(ctx, &id).await {
                add_playlist(ctx, cmd, guild_id, &data, list).await;
            } else {
                edit_embed(ctx, cmd, create_embed_no_yt_search_result()).await;
            }
        }

        PlayRequest::YouTubeVideoAndPlaylist { video, playlist } => {
            let song = yt_id_to_name(ctx, &video).await;
            let list = yt_list_id_to_vec(ctx, &playlist).await;

            if let (Some(song), Some(mut list)) = (song, list) {
                list.retain(|s| s.id != song.id);
                let playlist_len = list.len();

                let (req_len, play_len) = {
                    let mut server_props = data.all_ser_props.get(&guild_id).unwrap().write().await;
                    server_props.request_queue.push_back(song);
                    server_props.playlist_queue.append(&mut list);
                    server_props.playlist_queue_shuffle();
                    (
                        server_props.request_queue.len(),
                        server_props.playlist_queue.len(),
                    )
                };

                edit_embed(
                    ctx,
                    cmd,
                    create_embed_list_queue_added(1, req_len, playlist_len, play_len),
                )
                .await;
            } else {
                edit_embed(ctx, cmd, create_embed_no_yt_search_result()).await;
            }
        }

        PlayRequest::SpotifyTrack(id) => {
            if let Some(song) = data.spotify.get_track(ctx, &id).await {
                if let Some(song_searched) = yt_search(ctx, &song.title).await {
                    add_single_song(ctx, cmd, guild_id, &data, song_searched).await;
                } else {
                    edit_embed(ctx, cmd, create_embed_no_spotify_result()).await;
                }
            } else {
                edit_embed(ctx, cmd, create_embed_no_spotify_result()).await;
            }
        }

        PlayRequest::SpotifyPlaylist(id) => {
            if let Some(playlist) = data.spotify.get_playlist_tracks(ctx, &id).await {
                add_playlist(ctx, cmd, guild_id, &data, playlist).await;
            } else {
                edit_embed(ctx, cmd, create_embed_no_spotify_result()).await;
            }
        }

        PlayRequest::SpotifyAlbum(id) => {
            if let Some(album) = data.spotify.get_album_tracks(ctx, &id).await {
                add_playlist(ctx, cmd, guild_id, &data, album).await;
            } else {
                edit_embed(ctx, cmd, create_embed_no_spotify_result()).await;
            }
        }
    }

    audio_event(ctx, guild_id, voice_channel_id.unwrap()).await;
}

async fn add_single_song(
    ctx: &Context,
    cmd: &CommandInteraction,
    guild_id: GuildId,
    data: &BotData,
    song: Song,
) {
    let (is_playing, req_len, play_len) = {
        let mut server_props = data.all_ser_props.get(&guild_id).unwrap().write().await;
        server_props.request_queue.push_back(song.clone());
        (
            server_props.playing.is_some(),
            server_props.request_queue.len().to_string(),
            server_props.playlist_queue.len().to_string(),
        )
    };

    if is_playing {
        edit_embed(
            ctx,
            cmd,
            create_embed_user_queue_added(&song, req_len, play_len),
        )
        .await;
    } else {
        edit_embed(ctx, cmd, create_embed_now_playing(&song)).await;
    }
}

async fn add_playlist(
    ctx: &Context,
    cmd: &CommandInteraction,
    guild_id: GuildId,
    data: &BotData,
    mut playlist: VecDeque<Song>,
) {
    let playlist_len = playlist.len();

    let (req_len, play_len) = {
        let mut server_props = data.all_ser_props.get(&guild_id).unwrap().write().await;
        server_props.playlist_queue.append(&mut playlist);
        server_props.playlist_queue_shuffle();
        (
            server_props.request_queue.len(),
            server_props.playlist_queue.len(),
        )
    };

    edit_embed(
        ctx,
        cmd,
        create_embed_list_queue_added(0, req_len, playlist_len, play_len),
    )
    .await;
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("play")
        .description("Plays YouTube videos, playlists and Spotify tracks, albums and playlists")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "query",
                "Youtube or Spotify URL, or search",
            )
            .required(true),
        )
}
