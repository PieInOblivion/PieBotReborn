use crate::utils::audio_handler::audio_event;
use crate::utils::identify_source::parse_source;
use crate::utils::interaction::arg_to_str;
use crate::utils::query_youtube::{yt_id_to_name, yt_list_id_to_vec, yt_search};
use crate::utils::respond::{
    msg_list_queue_added, msg_no_spotify_result, msg_no_yt_search_result, msg_request_queue,
    msg_user_not_in_voice_channel,
};
use crate::utils::structs::{AllSerProps, Spotify};
use crate::utils::user_current_voice_and_guild::voice_and_guild;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::command::CommandOptionType;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction) {
    let (_, guild_id, voice_channel_id) = voice_and_guild(ctx, cmd);

    if voice_channel_id == None {
        msg_user_not_in_voice_channel(ctx, cmd).await;
        return;
    }

    let user_query: String = arg_to_str(cmd);
    let url_identify = parse_source(&user_query);

    let mut allserprops = {
        let data_read = ctx.data.read().await;
        data_read.get::<AllSerProps>().unwrap().clone()
    };
    let mut serprops = allserprops.get_mut(&guild_id).unwrap().write().await;

    if url_identify.search_needed {
        if let Some(song) = yt_search(&user_query).await {
            serprops.request_queue.push_back(song.clone());
            msg_request_queue(ctx, cmd, &serprops, song).await;
        } else {
            msg_no_yt_search_result(ctx, cmd, &user_query).await;
        }
    } else {
        if url_identify.yt_id.is_some() && url_identify.yt_list.is_some() {
            // && two 'if let Some()' statements is unstable in current rust
            // https://github.com/rust-lang/rust/issues/53667
            let song = yt_id_to_name(url_identify.yt_id.as_ref().unwrap()).await;
            let mut list = yt_list_id_to_vec(url_identify.yt_list.as_ref().unwrap()).await;
            if song.is_some() && list.is_some() {
                // Remove the duplicate song
                list.as_mut()
                    .unwrap()
                    .retain(|s| s.id != song.as_ref().unwrap().id);
                serprops.request_queue.push_back(song.unwrap());
                let len = list.as_ref().unwrap().len();
                serprops.playlist_queue.append(&mut list.unwrap());
                serprops.playlist_queue_shuffle();
                msg_list_queue_added(ctx, cmd, &serprops, 1, len).await;
            } else {
                msg_no_yt_search_result(ctx, cmd, &user_query).await;
            }
        }

        if url_identify.yt_id.is_some() && url_identify.yt_list.is_none() {
            if let Some(song) = yt_id_to_name(url_identify.yt_id.as_ref().unwrap()).await {
                serprops.request_queue.push_back(song.clone());
                msg_request_queue(ctx, cmd, &serprops, song).await;
            } else {
                msg_no_yt_search_result(ctx, cmd, &user_query).await;
            }
        }

        if url_identify.yt_list.is_some() && url_identify.yt_id.is_none() {
            if let Some(mut list) = yt_list_id_to_vec(url_identify.yt_list.as_ref().unwrap()).await
            {
                let len = list.len();
                serprops.playlist_queue.append(&mut list);
                serprops.playlist_queue_shuffle();
                msg_list_queue_added(ctx, cmd, &serprops, 0, len).await;
            } else {
                msg_no_yt_search_result(ctx, cmd, &user_query).await;
            }
        }

        if url_identify.spot_track.is_some()
            || url_identify.spot_list.is_some()
            || url_identify.spot_album.is_some()
        {
            let mut spotify = {
                let data_read = ctx.data.read().await;
                data_read.get::<Spotify>().unwrap().clone()
            };

            if let Some(id) = url_identify.spot_track {
                if let Some(song) = spotify.get_track(&id).await {
                    serprops.request_queue.push_back(song.clone());
                    msg_request_queue(ctx, cmd, &serprops, song).await;
                } else {
                    msg_no_spotify_result(ctx, cmd, &id).await;
                }
            }

            if let Some(id) = url_identify.spot_list {
                if let Some(mut playlist) = spotify.get_playlist_tracks(&id).await {
                    let len = playlist.len();
                    serprops.playlist_queue.append(&mut playlist);
                    serprops.playlist_queue_shuffle();
                    msg_list_queue_added(ctx, cmd, &serprops, 0, len).await;
                } else {
                    msg_no_spotify_result(ctx, cmd, &id).await;
                }
            }

            if let Some(id) = url_identify.spot_album {
                if let Some(mut album) = spotify.get_album_tracks(&id).await {
                    let len = album.len();
                    serprops.playlist_queue.append(&mut album);
                    serprops.playlist_queue_shuffle();
                    msg_list_queue_added(ctx, cmd, &serprops, 0, len).await;
                } else {
                    msg_no_spotify_result(ctx, cmd, &id).await;
                }
            }
        }
    }

    drop(serprops);

    audio_event(ctx, guild_id, voice_channel_id.unwrap()).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Plays YouTube videos, playlists and Spotify tracks, albums and playlists")
        .create_option(|option| {
            option
                .name("query")
                .description("Youtube or Spotify URL, or search")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
