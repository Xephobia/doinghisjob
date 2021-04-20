use serde_json::Value;
use serenity::{
    async_trait,
    framework::standard::{macros::hook, CommandResult, DispatchError},
    model::{channel::Message, id::GuildId},
    prelude::*,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        println!("bot created cache for {} guilds", _guilds.len());
    }

    async fn unknown(&self, _ctx: Context, _name: String, _raw: Value) {
        eprintln!("recieved uknown event\nname : {}\nvalue : {}", _name, _raw);
    }
}

#[hook]
pub async fn dispatch_error_hook(ctx: &Context, msg: &Message, err: DispatchError) {
    match err {
        DispatchError::CommandDisabled(command) => {
            msg.reply(ctx, format!("command {} disabled", command))
                .await
                .unwrap();
        }
        DispatchError::OnlyForDM => {
            msg.reply(ctx, "command only aviable for dm").await.unwrap();
        }
        DispatchError::OnlyForGuilds => {
            msg.reply(ctx, "command only aviable in guilds")
                .await
                .unwrap();
        }
        DispatchError::OnlyForOwners => {
            msg.reply(ctx, "you are not bot owner!").await.unwrap();
        }
        DispatchError::LackingPermissions(perms) => {
            msg.reply(ctx, format!("you do not have permisions {}", perms))
                .await
                .unwrap();
        }
        DispatchError::NotEnoughArguments { min, given } => {
            msg.reply(
                ctx,
                format!("not enough args : {} min, {} given", min, given),
            )
            .await
            .unwrap();
        }
        DispatchError::TooManyArguments { max, given } => {
            msg.reply(ctx, format!("too many args : {} max, {} given", max, given))
                .await
                .unwrap();
        }
        other => {
            msg.reply(ctx, format!("error occured : {:?}", other))
                .await
                .unwrap();
        }
    }
}

#[hook]
pub async fn after_hook(ctx: &Context, msg: &Message, cmd_name: &str, error: CommandResult) {
    if let Err(why) = error {
        msg.reply(ctx, format!("error in {} : ```{:#?}```", cmd_name, why))
            .await
            .unwrap();
    }
}
