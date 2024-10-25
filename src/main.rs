mod commands;
mod utils;

use crate::utils::spotify::Spotify;
use crate::utils::structs::{AllSerProps, SerProps};

use std::collections::HashMap;
use std::sync::Arc;

use serenity::prelude::TypeMapKey;
use tokio::sync::RwLock;

use reqwest::Client as HttpClient;

use serenity::all::{async_trait, Client, Context, EventHandler, GatewayIntents, GuildId, Interaction, Ready, VoiceState};

use songbird::SerenityInit;
use utils::reset_serprops::reset_serprops;


struct HttpKey;

impl TypeMapKey for HttpKey {
    type Value = HttpClient;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(cmd) = interaction {
            match cmd.data.name.as_str() {
                "play" => commands::play::run(&ctx, &cmd).await,
                "pause" => commands::pause::run(&ctx, &cmd).await,
                "resume" => commands::resume::run(&ctx, &cmd).await,
                "skip" => commands::skip::run(&ctx, &cmd).await,
                "stop" => commands::stop::run(&ctx, &cmd).await,
                "remove" => commands::remove::run(&ctx, &cmd).await,
                "np" => commands::now_playing::run(&ctx, &cmd).await,
                "queue" => commands::queue::run(&ctx, &cmd).await,
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
            let channel_id = match vs.channel_id {
                Some(id) => id,
                None => return
            };

            let guild_id = match vs.guild_id {
                Some(id) => id,
                None => return
            };

            let guild = guild_id.to_guild_cached(&ctx).unwrap().clone();
            let voice_states = guild.voice_states;

            let members_in_channel: Vec<&VoiceState> = voice_states
                .values()
                .filter(|state| state.channel_id == Some(channel_id))
                .collect();

            if members_in_channel.len() == 1 && members_in_channel[0].user_id == ctx.cache.current_user().id {
                reset_serprops(ctx, guild_id).await;
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let guilds_file = include_str!("../secret/guilds");

        for line in guilds_file.lines() {
            let gid = GuildId::new(line.parse().unwrap());

            let commands = vec![
                commands::play::register(),
                commands::pause::register(),
                commands::resume::register(),
                commands::skip::register(),
                commands::stop::register(),
                commands::remove::register(),
                commands::now_playing::register(),
                commands::queue::register(),
                commands::rps::register()
            ];

            let _commands = GuildId::set_commands(gid, &ctx.http, commands).await;
        }

        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let spotify_id = include_str!("../secret/spotifyId");
    let spotify_secret = include_str!("../secret/spotifySecret");
    let discord_token = include_str!("../secret/discord");
    let guilds_file = include_str!("../secret/guilds");

    let spotify = Spotify::new(spotify_id.to_string(), spotify_secret.to_string()).await;
    let mut allserprops: HashMap<GuildId, Arc<RwLock<SerProps>>> = HashMap::new();

    for line in guilds_file.lines() {
        let gid = GuildId::new(line.parse().unwrap());
        allserprops.insert(gid, Arc::new(RwLock::new(SerProps::new())));
    }

    let mut client = Client::builder(
        discord_token,
        GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILDS,
    )
    .event_handler(Handler)
    .register_songbird()
    .type_map_insert::<HttpKey>(HttpClient::new())
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
