mod commands;
mod utils;
use crate::utils::structs::{AllSerProps, SerProps};

use std::collections::HashMap;
use std::sync::Arc;

use songbird::SerenityInit;
use tokio::sync::RwLock;

use serenity::async_trait;
use serenity::client::{Context, EventHandler};
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::model::id::{ChannelId, GuildId};
use serenity::Client;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            // println!("Received command: {:#?}", command);

            let server_properties = {
                let data_read = ctx.data.read().await;
                data_read.get::<AllSerProps>().unwrap().clone()
            };

            let mut wait_write = server_properties.write().await;
            let serprops = wait_write.get_mut(&command.guild_id.unwrap()).unwrap();

            match command.data.name.as_str() {
                "play" => commands::play::run(&ctx, &command, serprops).await,
                "ping" => commands::ping::run(&ctx, &command).await,
                "rps" => commands::rps::run(&ctx, &command).await,
                _ => (),
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guilds_file = include_str!("../secret/channels");

        for line in guilds_file.lines() {
            let id: u64 = line[..18].parse().unwrap();
            let gid = GuildId(id);

            let _commands = GuildId::set_application_commands(&gid, &ctx.http, |commands| {
                commands
                    .create_application_command(|command| commands::play::register(command))
                    .create_application_command(|command| commands::ping::register(command))
                    .create_application_command(|command| commands::rps::register(command))
            })
            .await;

            // println!(
            //     "I now have the following guild slash commands: {:#?}",
            //     commands
            // );
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

    let mut client = Client::builder(
        token,
        GatewayIntents::GUILD_VOICE_STATES | GatewayIntents::GUILDS,
    )
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
