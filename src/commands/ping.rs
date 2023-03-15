use serenity::builder::CreateApplicationCommand;

pub fn run() -> String {
    "Hey, I'm alive!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("ping").description("A ping command")
}
