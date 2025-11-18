use serenity::all::CommandInteraction;

pub fn arg_to_str(cmd: &CommandInteraction) -> &str {
    cmd.data.options[0].value.as_str().unwrap()
}
