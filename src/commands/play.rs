use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(options: &[CommandDataOption]) -> String {
    let url: &str = options
        .get(0)
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    "Hey, I'm alive!".to_string()
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
