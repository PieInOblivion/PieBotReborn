use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub fn arg_to_str(cmd: &ApplicationCommandInteraction) -> String {
    cmd.data.options[0]
        .value
        .clone()
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
}
