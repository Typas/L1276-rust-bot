use serenity::{
    model::{channel::Message,
            id::{ChannelId, MessageId}},
    prelude::*,
    framework::standard::{
        Args, CommandResult, CommandError,
        macros::{command, group},
    },
};

use crate::util;

#[group]
#[commands(ping, pin_message, unpin_message)]
pub struct General;

#[command]
pub fn ping(ctx: &mut Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!")?;

    Ok(())
}

#[command]
#[aliases("pin")]
pub fn pin_message(ctx: &mut Context, msg: &Message, mut args: Args) -> CommandResult {
    let raw_str = args.raw().collect::<Vec<&str>>();
    let old_msg_id = util::arg_to_messageid(&raw_str[0])?;

    let channel_id = match raw_str.len() {
        0..=1 => msg.channel_id.clone(),
        _ => util::arg_to_channelid(&raw_str[1])?,
    };

    let old_msg = ctx.http.get_message(*channel_id.as_u64(), *old_msg_id.as_u64())?;

    old_msg.pin(&ctx.http)?;
    let reply_message = format!("Message \"{}\" has been pinned.", &old_msg.content_safe(&ctx.cache));

    if let Err(why) = msg.channel_id.say(&ctx.http, &reply_message) {
        eprintln!("Error sending message: {:?}", why);
    }

    Ok(())
}

#[command]
#[aliases("unpin")]
pub fn unpin_message(ctx: &mut Context, msg: &Message, args: Args) -> CommandResult {
    let raw_str = args.raw().collect::<Vec<&str>>();
    let old_msg_id = util::arg_to_messageid(&raw_str[0])?;
    let channel_id = match raw_str.len() {
        0..=1 => msg.channel_id.clone(),
        _ => util::arg_to_channelid(&raw_str[1])?,
    };

    let old_msg = ctx.http.get_message(*channel_id.as_u64(), *old_msg_id.as_u64())?;

    old_msg.unpin(&ctx.http)?;
    let reply_message = format!("Message \"{}\" has been unpinned.", &old_msg.content_safe(&ctx.cache));

    if let Err(why) = msg.channel_id.say(&ctx.http, &reply_message) {
        eprintln!("Error sending message: {:?}", why);
    }

    Ok(())
}
