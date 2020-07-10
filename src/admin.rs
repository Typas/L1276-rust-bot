use serenity::{
    model::channel::Message,
    client::Context,
    framework::standard::{
        Args, CheckResult, CommandOptions,
        macros::check,
    },
};

#[check]
#[name = "Admin"]
fn admin_check(ctx: &mut Context, msg: &Message, _: &mut Args, _: &CommandOptions) -> CheckResult {
    if let Some(member) = msg.member(&ctx.cache) {
        if let Ok(permissions)= member.permissions(&ctx.cache) {
            return permissions.administrator().into();
        }
    }

    false.into()
}
