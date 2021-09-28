use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult,
};
use serenity::model::channel::Message;

#[group]
#[prefixes("test")]
#[description = "Message Test"]
#[default_command(random_message)]
#[commands(dm, mention_me)]
struct Test;

#[command]
pub async fn random_message(_ctx: &Context, _msg: &Message) -> CommandResult {
    // TODO: send a random/static message to channel without mention anyone

    Ok(())
}

#[command]
pub async fn dm(ctx: &Context, msg: &Message) -> CommandResult {
    let dm = msg
        .author
        .dm(&ctx, |m| {
            m.content("安安");

            m
        })
        .await;

    if let Err(why) = dm {
        println!("Error when direct messaging user: {:?}", why);
    }

    Ok(())
}

#[command]
pub async fn mention_me(_ctx: &Context, _msg: &Message) -> CommandResult {
    // TODO: mention the original author

    Ok(())
}
