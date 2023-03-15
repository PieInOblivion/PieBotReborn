mod commands;

use std::collections::HashMap;
use std::sync::Arc;

use songbird::SerenityInit;
use tokio::sync::RwLock;

use serenity::async_trait;
use serenity::client::Context;
use serenity::client::EventHandler;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::GatewayIntents;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::ChannelId;
use serenity::prelude::TypeMapKey;
use serenity::Client;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let server_properties = {
                let data_read = ctx.data.read().await;
                data_read.get::<AllSerProps>().unwrap().clone()
            };

            let mut wait_write = server_properties.write().await;
            let mut serprops = wait_write.get_mut(&command.guild_id.unwrap()).unwrap();

            let content = match command.data.name.as_str() {
                "play" => commands::play::run(&command.data.options),
                "ping" => commands::ping::run(),
                "rps" => commands::rps::run(&command.data.options),
                _ => "not implemented :(".to_string(),
            };

            if let Err(err) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", err);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guilds_file = include_str!("../secret/channels");

        for line in guilds_file.lines() {
            let id: u64 = line[..18].parse().unwrap();
            let gid = GuildId(id);

            let commands = GuildId::set_application_commands(&gid, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::play::register(command))
                    .create_application_command(|command| commands::ping::register(command))
                    .create_application_command(|command| commands::rps::register(command))
            })
            .await;

            println!(
                "I now have the following guild slash commands: {:#?}",
                commands
            );
        }
    }
}

#[tokio::main]
async fn main() {
    let token = include_str!("../secret/discord");

    let guilds_file = include_str!("../secret/channels");

    let sp: Arc<RwLock<HashMap<GuildId, SerProps>>> = Arc::new(RwLock::new(HashMap::new()));

    for line in guilds_file.lines() {
        let guild: GuildId = GuildId(line[..18].parse().unwrap());
        let channel: ChannelId = line[19..].parse().unwrap();
        let mut n = sp.write().await;
        n.insert(guild, SerProps::new(channel));
    }

    let mut client = Client::builder(token, GatewayIntents::GUILD_VOICE_STATES)
        .event_handler(Handler)
        .register_songbird()
        .await
        .expect("Error creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<AllSerProps>(sp);
    }

    if let Err(err) = client.start().await {
        println!("Client error: {:?}", err);
    }
}

struct AllSerProps;

#[derive(Debug)]
pub struct SerProps {
    channel_id: ChannelId,
    user_queue: Vec<Song>,
    playlist_queue: Vec<Song>,
    playing: Option<Song>,
    repeat: bool,
}

impl TypeMapKey for AllSerProps {
    type Value = Arc<RwLock<HashMap<GuildId, SerProps>>>;
}

impl SerProps {
    fn new(channel_id: ChannelId) -> SerProps {
        return SerProps {
            channel_id,
            user_queue: Vec::new(),
            playlist_queue: Vec::new(),
            playing: None,
            repeat: false,
        };
    }
}

#[derive(Debug)]
struct Song {
    id: String,
    requires_search: bool,
}
