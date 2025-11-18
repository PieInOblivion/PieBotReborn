use crate::utils::audio_handler::audio_event;
use crate::utils::identify_source::parse_source;
use crate::utils::interaction::arg_to_str;
use crate::utils::respond::{
    msg_list_queue_added, msg_no_spotify_result, msg_no_yt_search_result, msg_request_queue,
    msg_user_not_in_voice_channel,
};
use crate::utils::structs::BotData;
use crate::utils::user_current_voice_and_guild::voice_and_guild;
use crate::utils::youtube::{yt_id_to_name, yt_list_id_to_vec, yt_search};

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
};

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let (_, guild_id, voice_channel_id) = voice_and_guild(ctx, cmd);

    if voice_channel_id.is_none() {
        msg_user_not_in_voice_channel(ctx, cmd).await;
        return;
    }

    let user_query: String = arg_to_str(cmd);
    let url_identify = parse_source(&user_query);

    let data = ctx.data::<BotData>();
    let mut server_props = data.all_ser_props.get(&guild_id).unwrap().write().await;

    if url_identify.search_needed {
        if let Some(song) = yt_search(ctx, &user_query).await {
            msg_request_queue(ctx, cmd, &server_props, song.clone()).await;
            server_props.request_queue.push_back(song);
        } else {
            msg_no_yt_search_result(ctx, cmd, &user_query).await;
        }
    } else {
        if url_identify.yt_id.is_some() && url_identify.yt_list.is_some() {
            // && two 'if let Some()' statements is unstable in current rust
            // https://github.com/rust-lang/rust/issues/53667
            let yt_id = url_identify.yt_id.as_ref().unwrap().as_str();
            let yt_list = url_identify.yt_list.as_ref().unwrap().as_str();
            let song = yt_id_to_name(ctx, yt_id).await;
            let list = yt_list_id_to_vec(ctx, yt_list).await;
            if let (Some(song), Some(mut list)) = (song, list) {
                // Remove the duplicate song
                list.retain(|s| s.id != song.id);
                server_props.request_queue.push_back(song);
                let len = list.len();
                server_props.playlist_queue.append(&mut list);
                server_props.playlist_queue_shuffle();
                msg_list_queue_added(ctx, cmd, &server_props, 1, len).await;
            } else {
                msg_no_yt_search_result(ctx, cmd, &user_query).await;
            }
        }

        if url_identify.yt_id.is_some() && url_identify.yt_list.is_none() {
            let yt_id = url_identify.yt_id.as_ref().unwrap().as_str();
            if let Some(song) = yt_id_to_name(ctx, yt_id).await {
                msg_request_queue(ctx, cmd, &server_props, song.clone()).await;
                server_props.request_queue.push_back(song);
            } else {
                msg_no_yt_search_result(ctx, cmd, &user_query).await;
            }
        }

        if url_identify.yt_list.is_some() && url_identify.yt_id.is_none() {
            let yt_list = url_identify.yt_list.as_ref().unwrap().as_str();
            if let Some(mut list) = yt_list_id_to_vec(ctx, yt_list).await {
                let len = list.len();
                server_props.playlist_queue.append(&mut list);
                server_props.playlist_queue_shuffle();
                msg_list_queue_added(ctx, cmd, &server_props, 0, len).await;
            } else {
                msg_no_yt_search_result(ctx, cmd, &user_query).await;
            }
        }

        if url_identify.spot_track.is_some()
            || url_identify.spot_list.is_some()
            || url_identify.spot_album.is_some()
        {
            let spotify = &data.spotify;

            if let Some(id) = url_identify.spot_track {
                if let Some(song) = spotify.get_track(ctx, id.as_str()).await {
                    if let Some(song_searched) = yt_search(ctx, &song.title).await {
                        msg_request_queue(ctx, cmd, &server_props, song_searched.clone()).await;
                        server_props.request_queue.push_back(song_searched);
                    } else {
                        msg_no_spotify_result(ctx, cmd, &id).await;
                    }
                } else {
                    msg_no_spotify_result(ctx, cmd, &id).await;
                }
            }

            if let Some(id) = url_identify.spot_list {
                if let Some(mut playlist) = spotify.get_playlist_tracks(ctx, id.as_str()).await {
                    let len = playlist.len();
                    server_props.playlist_queue.append(&mut playlist);
                    server_props.playlist_queue_shuffle();
                    msg_list_queue_added(ctx, cmd, &server_props, 0, len).await;
                } else {
                    msg_no_spotify_result(ctx, cmd, &id).await;
                }
            }

            if let Some(id) = url_identify.spot_album {
                if let Some(mut album) = spotify.get_album_tracks(ctx, id.as_str()).await {
                    let len = album.len();
                    server_props.playlist_queue.append(&mut album);
                    server_props.playlist_queue_shuffle();
                    msg_list_queue_added(ctx, cmd, &server_props, 0, len).await;
                } else {
                    msg_no_spotify_result(ctx, cmd, &id).await;
                }
            }
        }
    }

    drop(server_props);

    audio_event(ctx, guild_id, voice_channel_id.unwrap()).await;
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
