use serenity::client::Context;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult,
};
use serenity::model::channel::Message;
use serenity::utils::MessageBuilder;

use crate::util;

#[group]
#[commands(ping, pin_message, unpin_message)]
pub struct General;

#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
#[aliases("pin")]
pub async fn pin_message(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let raw_str = args.raw().collect::<Vec<&str>>();
    let old_msg_id = util::arg_to_messageid(&raw_str[0])?;

    let channel_id = match raw_str.len() {
        0..=1 => msg.channel_id.clone(),
        _ => util::arg_to_channelid(&raw_str[1])?,
    };

    let old_msg = ctx
        .http
        .get_message(*channel_id.as_u64(), *old_msg_id.as_u64())
        .await?;

    old_msg.pin(&ctx.http).await?;
    let record: String = MessageBuilder::new()
        .push("Message ")
        .push_mono_safe(&old_msg.content)
        .push(" has been pinned.")
        .build();
    println!("{}", record);

    Ok(())
}

#[command]
#[aliases("unpin")]
pub async fn unpin_message(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let raw_str = args.raw().collect::<Vec<&str>>();
    let old_msg_id = util::arg_to_messageid(&raw_str[0])?;
    let channel_id = match raw_str.len() {
        0..=1 => msg.channel_id.clone(),
        _ => util::arg_to_channelid(&raw_str[1])?,
    };

    let old_msg = ctx
        .http
        .get_message(*channel_id.as_u64(), *old_msg_id.as_u64())
        .await?;

    old_msg.unpin(&ctx.http).await?;

    let record: String = MessageBuilder::new()
        .push("Message ")
        .push_mono_safe(&old_msg.content)
        .push(" has been unpinned.")
        .build();
    println!("{}", record);

    if let Err(why) = msg.channel_id.say(&ctx.http, &record).await {
        eprintln!("Error sending message: {:?}", why);
    }

    Ok(())
}
