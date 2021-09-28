use serenity::{
    client::Context,
    framework::standard::{macros::check, Args, CommandOptions, Reason},
    model::channel::Message,
};

#[check]
#[name = "Admin"]
pub async fn admin_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    match msg.member(&ctx).await {
        Ok(member) => match member.permissions(&ctx).await {
            Ok(perm) => match perm.administrator() {
                true => Ok(()),
                false => Err(Reason::User("Not an administrator".to_string())),
            },
            Err(e) => Err(Reason::User(e.to_string())),
        },
        Err(e) => Err(Reason::User(e.to_string())),
    }
}
