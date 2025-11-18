use crate::utils::respond::msg_rps;

use rand::Rng;

use std::fs;

use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
};

pub async fn run(ctx: &Context, cmd: &CommandInteraction) {
    let raw_file = fs::read("./secret/rps").unwrap();
    let file_to_string = String::from_utf8_lossy(&raw_file);
    let mut history_str = file_to_string
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap());

    let bot_score: u32 = history_str.next().unwrap();
    let usr_score: u32 = history_str.next().unwrap();

    let usr_choice: &str = cmd.data.options[0].value.as_str().unwrap();

    let bot_choice: &str = ["Rock", "Paper", "Scissors"][rand::rng().random_range(0..=2)];

    match (bot_choice, usr_choice) {
        ("Rock", "Scissors") | ("Scissors", "Paper") | ("Paper", "Rock") => {
            save_rps(bot_score + 1, usr_score);
            msg_rps(ctx, cmd, bot_score + 1, usr_score, "I won!").await;
        }
        ("Rock", "Paper") | ("Scissors", "Rock") | ("Paper", "Scissors") => {
            save_rps(bot_score, usr_score + 1);
            msg_rps(ctx, cmd, bot_score, usr_score + 1, "You won!").await;
        }
        _ => msg_rps(ctx, cmd, bot_score, usr_score, "We tied!").await,
    }
}

pub fn register() -> CreateCommand<'static> {
    CreateCommand::new("rps")
        .description("Rock, Paper, Scissors!")
        .add_option(
            CreateCommandOption::new(CommandOptionType::String, "choice", "Your choice")
                .required(true)
                .add_string_choice("Rock", "Rock")
                .add_string_choice("Paper", "Paper")
                .add_string_choice("Scissors", "Scissors"),
        )
}

fn save_rps(bot_score: u32, usr_score: u32) {
    fs::write("./secret/rps", format!("{bot_score} {usr_score}")).unwrap();
}
