mod commands;
mod utils;

use crate::utils::spotify::Spotify;
use crate::utils::structs::{BotData, ServerProps};

use std::collections::HashMap;
use std::sync::Arc;

use serenity::prelude::{Context, EventHandler};
use tokio::sync::RwLock;

use reqwest::Client as HttpClient;

use serenity::all::{
    Client, FullEvent, GatewayIntents, GuildId, Interaction, VoiceState, async_trait,
};
use songbird::Songbird;

use utils::reset_serprops::reset_serprops;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn dispatch(&self, ctx: &Context, event: &FullEvent) {
        match event {
            FullEvent::InteractionCreate {
                interaction: Interaction::Command(cmd),
                ..
            } => match cmd.data.name.as_str() {
                "play" => commands::play::run(ctx, cmd).await,
                "pause" => commands::pause::run(ctx, cmd).await,
                "resume" => commands::resume::run(ctx, cmd).await,
                "skip" => commands::skip::run(ctx, cmd).await,
                "stop" => commands::stop::run(ctx, cmd).await,
                "remove" => commands::remove::run(ctx, cmd).await,
                "np" => commands::now_playing::run(ctx, cmd).await,
                "queue" => commands::queue::run(ctx, cmd).await,
                "rps" => commands::rps::run(ctx, cmd).await,
                _ => (),
            },
            FullEvent::InteractionCreate { .. } => {}
            FullEvent::VoiceStateUpdate { old, new, .. } => {
                if let Some(old_state) = old {
                    all_alone_check_and_leave(ctx, old_state).await;
                }

                all_alone_check_and_leave(ctx, new).await;

                async fn all_alone_check_and_leave(ctx: &Context, vs: &VoiceState) {
                    let channel_id = match vs.channel_id {
                        Some(id) => id,
                        None => return,
                    };

                    let guild_id = match vs.guild_id {
                        Some(id) => id,
                        None => return,
                    };

                    let alone_in_channel = {
                        let guild = guild_id.to_guild_cached(&ctx.cache).unwrap();
                        let voice_states = &guild.voice_states;

                        let members_in_channel: Vec<&VoiceState> = voice_states
                            .into_iter()
                            .filter(|state| state.channel_id == Some(channel_id))
                            .collect();

                        members_in_channel.len() == 1
                            && members_in_channel[0].user_id == ctx.cache.current_user().id
                    };

                    if alone_in_channel {
                        reset_serprops(ctx, guild_id).await;
                    }
                }
            }
            FullEvent::Ready { data_about_bot, .. } => {
                let data = ctx.data::<BotData>();
                let guild_ids: Vec<GuildId> = data.all_ser_props.keys().copied().collect();

                for gid in guild_ids {
                    let commands = vec![
                        commands::play::register(),
                        commands::pause::register(),
                        commands::resume::register(),
                        commands::skip::register(),
                        commands::stop::register(),
                        commands::remove::register(),
                        commands::now_playing::register(),
                        commands::queue::register(),
                        commands::rps::register(),
                    ];

                    let _commands = GuildId::set_commands(gid, &ctx.http, &commands).await;
                }

                println!("{} is connected!", data_about_bot.user.name);
            }
            _ => {}
        }
    }
}

#[tokio::main]
async fn main() {
    let spotify_id = include_str!("../secret/spotifyId");
    let spotify_secret = include_str!("../secret/spotifySecret");
    let discord_token = include_str!("../secret/discord");
    let guilds_file = include_str!("../secret/guilds");

    // Parse guilds once and reuse
    let guild_ids: Vec<GuildId> = guilds_file
        .lines()
        .map(|line| GuildId::new(line.parse().unwrap()))
        .collect();

    let spotify = Spotify::new(spotify_id.to_string(), spotify_secret.to_string()).await;
    let mut allserprops: HashMap<GuildId, Arc<RwLock<ServerProps>>> = HashMap::new();

    for gid in &guild_ids {
        allserprops.insert(*gid, Arc::new(RwLock::new(ServerProps::new())));
    }

    let songbird = Songbird::serenity();

    let bot_data = Arc::new(BotData {
        all_ser_props: allserprops,
        spotify,
        http: HttpClient::new(),
        songbird: songbird.clone(),
    });

    let mut client = Client::builder(
        discord_token.trim().parse().expect("Invalid token"),
        GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILDS,
    )
    .event_handler(Handler)
    .voice_manager::<Songbird>(songbird)
    .data(bot_data)
    .await
    .expect("Error creating client");

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}
