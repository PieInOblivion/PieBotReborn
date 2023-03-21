use crate::utils::identify_source::parse_source;
use crate::utils::interaction::arg_to_str;
use crate::utils::respond::msg_user_not_in_voice_channel;
use crate::utils::structs::SerProps;

use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;

pub async fn run(ctx: &Context, cmd: &ApplicationCommandInteraction, serprops: &mut SerProps) {
    // check if user is in voice channel
    // check if currently in that voice channel
    //     if not, join, AND parse user input to song items
    //     if is, just parse user input
    // place parsed song items into relevant queue
    //

    // Check if user is in a voice channel
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

    // Parse the user input
    let url: String = arg_to_str(cmd);
    let url_identify = parse_source(&url);

    // Convert parsed information into serprops
    if url_identify.search_needed {
    } else {
        if url_identify.yt_id.is_some() {}
        if url_identify.yt_list.is_some() {}
        if url_identify.spot_track.is_some() {}
        if url_identify.spot_list.is_some() {}
        if url_identify.spot_album.is_some() {}
    }

    // Retrieve global songbird manager
    let manager = songbird::get(ctx).await.unwrap();

    // Check if already in the voice channel with the latest request
    if let Some(guild_connection) = manager.get(guild_id) {
        // Is in this guild. Check if same voice channel
        let mut call = guild_connection.lock().await;
        if call.current_channel().unwrap().0 != voice_channel_id.unwrap().0 {
            // In same guild but not channel, move channels
            let _ = call.join(voice_channel_id.unwrap()).await;
        }
    } else {
        // TODO: deal with error joining
        // Since not in the guild yet, join
        let _ = manager.join(guild_id, voice_channel_id.unwrap()).await;
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:#?}", why);
                return;
            }
        };

        handler.play_source(source);
    } else {
        println!("not in voice channel");
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("play")
        .description("Plays YouTube videos, playlists and Spotify tracks, albums and playlists")
        .create_option(|option| {
            option
                .name("url")
                .description("URL of the source")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
