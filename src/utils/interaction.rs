use serenity::all::CommandInteraction;

pub fn arg_to_str(cmd: &CommandInteraction) -> String {
    cmd.data.options[0].value.as_str().unwrap().to_string()
}
