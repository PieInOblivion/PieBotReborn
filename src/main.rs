mod commands;
mod utils;

use crate::utils::spotify::Spotify;
use crate::utils::structs::{BotData, ServerProps};

use std::collections::HashMap;
use std::env;
use std::fs;
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
                if let Some(old_state) = old
                    && let Some(guild_id) = guild_id_if_alone(ctx, old_state)
                {
                    reset_serprops(ctx, guild_id).await;
                    return;
                }

                if let Some(guild_id) = guild_id_if_alone(ctx, new) {
                    reset_serprops(ctx, guild_id).await;
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

fn guild_id_if_alone(ctx: &Context, vs: &VoiceState) -> Option<GuildId> {
    let channel_id = vs.channel_id?;

    let guild_id = vs.guild_id?;

    let guild = guild_id.to_guild_cached(&ctx.cache).unwrap();
    let voice_states = &guild.voice_states;

    let bot_id = ctx.cache.current_user().id;

    let mut channel_members = voice_states
        .iter()
        .filter(|state| state.channel_id == Some(channel_id));

    match (channel_members.next(), channel_members.next()) {
        (Some(only_member), None) if only_member.user_id == bot_id => Some(guild_id),
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let spotify_id = env::var("SPOTIFY_ID").expect("SPOTIFY_ID env not set");
    let spotify_secret = env::var("SPOTIFY_SECRET").expect("SPOTIFY_SECRET env not set");
    let discord_token = env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN env not set");
    let youtube_key = env::var("YOUTUBE_KEY").expect("YOUTUBE_KEY env not set");

    let guilds_file = fs::read_to_string("secret/guilds").expect("Failed to read secret/guilds");

    let guild_ids: Vec<GuildId> = guilds_file
        .lines()
        .map(|line| GuildId::new(line.parse().unwrap()))
        .collect();

    let spotify = Spotify::new(spotify_id, spotify_secret).await;
    let mut allserprops: HashMap<GuildId, RwLock<ServerProps>> = HashMap::new();

    for gid in &guild_ids {
        allserprops.insert(*gid, RwLock::new(ServerProps::new()));
    }

    let songbird = Songbird::serenity();

    let bot_data = Arc::new(BotData {
        all_ser_props: allserprops,
        spotify,
        http: HttpClient::new(),
        songbird: songbird.clone(),
        youtube_key,
    });

    let mut client = Client::builder(
        discord_token.trim().parse().expect("Invalid token"),
        GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILDS,
    )
    .event_handler(Arc::new(Handler))
    .voice_manager(songbird)
    .data(bot_data)
    .await
    .expect("Error creating client");

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}
