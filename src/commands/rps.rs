use rand::Rng;

use std::fs;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(options: &[CommandDataOption]) -> String {
    let raw_file = fs::read("./secret/rps").unwrap();
    let file_to_string = String::from_utf8_lossy(&raw_file);
    let mut history_str = file_to_string
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap());

    let bot_score: u32 = history_str.next().unwrap();
    let usr_score: u32 = history_str.next().unwrap();

    let usr_choice: &str = options
        .get(0)
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let bot_choice: &str = ["Rock", "Paper", "Scissors"][rand::thread_rng().gen_range(0..=2)];

    match (bot_choice, usr_choice) {
        ("Rock", "Scissors") | ("Scissors", "Paper") | ("Paper", "Rock") => {
            save_rps(bot_score + 1, usr_score);
            format!("Bot Wins!\nBot: {} | You: {}", bot_score + 1, usr_score)
        }
        ("Rock", "Paper") | ("Scissors", "Rock") | ("Paper", "Scissors") => {
            save_rps(bot_score, usr_score + 1);
            format!("You Wins!\nBot: {} | You: {}", bot_score, usr_score + 1)
        }
        _ => format!("Tie!\nBot: {bot_score} | You: {usr_score}"),
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("rps")
        .description("Rock, Paper, Scissors!")
        .create_option(|option| {
            option
                .name("choice")
                .description("Your choice")
                .required(true)
                .kind(CommandOptionType::String)
                .add_string_choice("Rock", "Rock")
                .add_string_choice("Paper", "Paper")
                .add_string_choice("Scissors", "Scissors")
        })
}

fn save_rps(bot_score: u32, usr_score: u32) {
    fs::write("./secret/rps", format!("{bot_score} {usr_score}")).unwrap();
}
