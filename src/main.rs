mod commands;
mod utils;

use crate::utils::spotify::Spotify;
use crate::utils::structs::{AllSerProps, SerProps};

use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;

use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::id::{ChannelId, GuildId};
use serenity::model::voice::VoiceState;
use serenity::Client;

use songbird::SerenityInit;
use utils::reset_serprops::reset_serprops;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(cmd) = interaction {
            match cmd.data.name.as_str() {
                "play" => commands::play::run(&ctx, &cmd).await,
                "pause" => commands::pause::run(&ctx, &cmd).await,
                "resume" => commands::resume::run(&ctx, &cmd).await,
                "skip" => commands::skip::run(&ctx, &cmd).await,
                "stop" => commands::stop::run(&ctx, &cmd).await,
                "remove" => commands::remove::run(&ctx, &cmd).await,
                "np" => commands::now_playing::run(&ctx, &cmd).await,
                "queue" => commands::queue::run(&ctx, &cmd).await,
                "ping" => commands::ping::run(&ctx, &cmd).await,
                "rps" => commands::rps::run(&ctx, &cmd).await,
                _ => (),
            };
        }
    }

    async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
        if let Some(old_state) = old {
            all_alone_check_and_leave(&ctx, old_state).await;
        }

        all_alone_check_and_leave(&ctx, new).await;

        async fn all_alone_check_and_leave(ctx: &Context, vs: VoiceState) {
            if let Some(channel_id) = vs.channel_id {
                if let Some(channel) = ctx.cache.guild_channel(channel_id) {
                    if let Ok(user_list) = channel.members(&ctx.cache).await {
                        if user_list.len() == 1
                            && user_list
                                .iter()
                                .any(|user| user.user.id == ctx.cache.current_user_id())
                        {
                            reset_serprops(ctx, channel.guild_id).await;
                        }
                    }
                }
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let guilds_file = include_str!("../secret/channels");

        for line in guilds_file.lines() {
            let id: u64 = line[..18].parse().unwrap();
            let gid = GuildId(id);

            let _commands = GuildId::set_application_commands(&gid, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::play::register(command))
                    .create_application_command(|command| commands::pause::register(command))
                    .create_application_command(|command| commands::resume::register(command))
                    .create_application_command(|command| commands::skip::register(command))
                    .create_application_command(|command| commands::stop::register(command))
                    .create_application_command(|command| commands::remove::register(command))
                    .create_application_command(|command| commands::now_playing::register(command))
                    .create_application_command(|command| commands::queue::register(command))
                    .create_application_command(|command| commands::ping::register(command))
                    .create_application_command(|command| commands::rps::register(command))
            })
            .await;
        }

        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let spotify_id = include_str!("../secret/spotifyId");
    let spotify_secret = include_str!("../secret/spotifySecret");
    let discord_token = include_str!("../secret/discord");
    let guilds_file = include_str!("../secret/channels");

    let spotify = Spotify::new(spotify_id.to_string(), spotify_secret.to_string()).await;
    let mut allserprops: HashMap<GuildId, Arc<RwLock<SerProps>>> = HashMap::new();

    for line in guilds_file.lines() {
        let guild: GuildId = GuildId(line[..18].parse().unwrap());
        let channel: ChannelId = line[19..].parse().unwrap();
        allserprops.insert(guild, Arc::new(RwLock::new(SerProps::new(channel))));
    }

    let mut client = Client::builder(
        discord_token,
        GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILDS,
    )
    .event_handler(Handler)
    .register_songbird()
    .await
    .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<AllSerProps>(allserprops);
        data.insert::<Spotify>(spotify);
    }

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}
