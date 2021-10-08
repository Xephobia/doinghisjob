use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
    utils::ArgumentConvert,
};

#[command]
#[description = "kick user"]
#[required_permissions(KICK_MEMBERS)]
pub(crate) async fn kick(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    for u in args.raw() {
        match Member::convert(ctx, msg.guild_id, Some(msg.channel_id), u).await {
            Ok(m) => m.kick(ctx).await?,
            Err(e) => println!("{}", e),
        }
    }

    Ok(())
}
