use crate::utils::audio_handler::audio_event;
use crate::utils::identify_source::parse_source;
use crate::utils::interaction::arg_to_str;
use crate::utils::query_youtube::{yt_id_to_name, yt_list_id_to_vec, yt_search};
use crate::utils::respond::{
    msg_list_queue_added, msg_no_yt_search_result, msg_request_queue, msg_user_not_in_voice_channel,
};
use crate::utils::shuffle_vec::shuffle_vec;
use crate::utils::structs::SerProps;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction, serprops: &mut SerProps) {
    let guild_id = cmd.guild_id.unwrap();
    let guild = ctx.cache.guild(guild_id).unwrap();

    let voice_channel_id = guild
        .voice_states
        .get(&cmd.user.id)
        .and_then(|vs| vs.channel_id);

    if voice_channel_id == None {
        msg_user_not_in_voice_channel(ctx, cmd).await;
        return;
    }

    let user_query: String = arg_to_str(cmd);
    let url_identify = parse_source(&user_query);

    if url_identify.search_needed {
        if let Some(song) = yt_search(&user_query).await {
            serprops.request_queue.push(song.clone());
            msg_request_queue(ctx, cmd, serprops, &song).await;
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
                serprops.request_queue.push(song.unwrap());
                let len = list.as_ref().unwrap().len();
                serprops.playlist_queue.append(&mut list.unwrap());
                shuffle_vec(&mut serprops.playlist_queue);
                msg_list_queue_added(ctx, cmd, serprops, 1, len).await;
            } else {
                msg_no_yt_search_result(ctx, cmd, &user_query).await;
            }
        }

        if url_identify.yt_id.is_some() && url_identify.yt_list.is_none() {
            if let Some(song) = yt_id_to_name(url_identify.yt_id.as_ref().unwrap()).await {
                serprops.request_queue.push(song.clone());
                msg_request_queue(ctx, cmd, serprops, &song).await;
            } else {
                msg_no_yt_search_result(ctx, cmd, &user_query).await;
            }
        }

        if url_identify.yt_list.is_some() && url_identify.yt_id.is_none() {
            if let Some(mut list) = yt_list_id_to_vec(url_identify.yt_list.as_ref().unwrap()).await
            {
                let len = list.len();
                serprops.playlist_queue.append(&mut list);
                shuffle_vec(&mut serprops.playlist_queue);
                msg_list_queue_added(ctx, cmd, serprops, 0, len).await;
            } else {
                msg_no_yt_search_result(ctx, cmd, &user_query).await;
            }
        }

        if url_identify.spot_track.is_some() {}

        if url_identify.spot_list.is_some() {}

        if url_identify.spot_album.is_some() {}
    }

    audio_event(ctx, serprops, guild_id, voice_channel_id.unwrap()).await;
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
