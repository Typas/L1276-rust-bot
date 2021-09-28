use serenity::client::Context;
use serenity::framework::standard::macros::hook;
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;

#[hook]
pub async fn before(_ctx: &Context, msg: &Message, cmd_name: &str) -> bool {
    println!("Got command '{}' from user '{}'", cmd_name, msg.author.name);
    true
}

#[hook]
pub async fn after(_: &Context, _: &Message, cmd_name: &str, error: Result<(), CommandError>) {
    match error {
        Ok(()) => println!("Processed command '{}'", cmd_name),
        Err(why) => println!("Command '{}' returned error: {:?}", cmd_name, why),
    }
}

#[hook]
pub async fn unrecognised_command(_: &Context, _: &Message, unknown_cmd_name: &str) {
    println!("Could not find command named '{}'", unknown_cmd_name);
}

#[hook]
pub async fn normal_message(_: &Context, msg: &Message) {
    println!("Message '{}' is not a command", msg.content);
}
