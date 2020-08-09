use serenity::{
    client::Context,
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::channel::Message,
};

#[group]
#[prefixes("test")]
#[description = "Message Test"]
#[default_command(random_message)]
#[commands(dm, mention_me)]
struct Test;

#[command]
fn random_message(_ctx: &mut Context, _msg: &Message) -> CommandResult {
    // TODO: send a random/static message to channel without mention anyone

    Ok(())
}

#[command]
fn dm(ctx: &mut Context, msg: &Message) -> CommandResult {
    let dm = msg.author.dm(&ctx, |m| {
        m.content("安安");

        m
    });

    if let Err(why) = dm {
        println!("Error when direct messaging user: {:?}", why);
    }

    Ok(())
}

#[command]
fn mention_me(_ctx: &mut Context, _msg: &Message) -> CommandResult {
    // TODO: mention the original author

    Ok(())
}
