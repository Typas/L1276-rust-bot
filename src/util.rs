use serenity::{
    model::id::{ChannelId, MessageId, RoleId, UserId},
    framework::standard::CommandError,
};
use regex::Regex;

pub type IdResult<T> = Result<T, CommandError>;

// pure u64
pub fn arg_to_messageid(arg: &str) -> IdResult<MessageId> {
    let n = arg.parse::<u64>()?;
    let id = MessageId::from(n);
    Ok(id)
}

// <#id>
pub fn arg_to_channelid(arg: &str) -> IdResult<ChannelId> {
    let r = Regex::new("<#[0-9]+>").unwrap();
    let n = regex_find_u64(arg, &r)?;
    let id = ChannelId::from(n);
    Ok(id)
}

// <@&id>
pub fn arg_to_roleid(arg: &str) -> IdResult<RoleId> {
    let r = Regex::new("<@*&[0-9]+>").unwrap();
    let n = regex_find_u64(arg, &r)?;
    let id = RoleId::from(n);
    Ok(id)
}

// <@!id>
pub fn arg_to_userid(arg: &str) -> IdResult<UserId> {
    let r = Regex::new("<@*![0-9]+>").unwrap();
    let n = regex_find_u64(arg, &r)?;
    let id = UserId::from(n);
    Ok(id)
}

pub fn regex_find_u64(arg: &str, re: &Regex) -> IdResult<u64> {
    let err = CommandError(format!("Cannot parse from argument {}", arg));
    let r = Regex::new("[0-9]+").unwrap();

    let tmp = re.find(arg);
    let s = match tmp {
        Some(t) => r.find(arg),
        None => return Err(err),
    };
    let n = match s {
        Some(s) => s.as_str().parse::<u64>(),
        None => return Err(err),
    };

    match n {
        Ok(n) => Ok(n),
        Err(_) => Err(err),
    }
}
