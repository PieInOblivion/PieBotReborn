use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;

pub fn arg_to_str(cmd: &ApplicationCommandInteraction) -> String {
    let arg_zero: String = cmd.data.options[0].value.clone().unwrap().to_string();
    let mut chars = arg_zero.chars();
    chars.next();
    chars.next_back();
    chars.collect::<String>()
}
