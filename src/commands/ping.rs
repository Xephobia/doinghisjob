use anyhow::anyhow;
use serenity::{
    client::bridge::gateway::{ShardId, ShardManager},
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::sync::Arc;

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[command]
#[aliases("ping")]
#[description = "get the shard's ping"]
pub async fn latency(ctx: &Context, msg: &Message) -> CommandResult {
    let context_data = &ctx.data.read().await;
    let shard_manager = match context_data.get::<ShardManagerContainer>() {
        Some(shrd_manage) => shrd_manage,
        None => return Err(anyhow!("error in ^ping | failed to get shard manager").into()),
    };
    let manager = shard_manager.lock().await;
    let runners = manager.runners.lock().await;
    let runner = match runners.get(&ShardId(ctx.shard_id)) {
        Some(runner) => runner,
        None => return Err(anyhow!("error in ^ping | failed to get shard").into()),
    };
    let shard_latency = runner.latency.unwrap().as_millis();
    msg.reply(ctx, format!("pong! {}ms", shard_latency)).await?;
    Ok(())
}
