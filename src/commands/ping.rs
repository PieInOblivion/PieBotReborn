use crate::utils::respond::msg_ping;
use serenity::builder::CreateApplicationCommand;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub async fn run(ctx: &Context, msg: &ApplicationCommandInteraction) {
    msg_ping(ctx, msg).await;
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
