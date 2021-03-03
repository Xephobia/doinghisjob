use serenity::{client::bridge::gateway::{ShardId, ShardManager}, framework::standard::{CommandResult, macros::{command, group}}, model::prelude::*, prelude::*};
use std::{sync::Arc};

pub struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}

#[command]
#[description = "get the shard's ping"]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
  let context_data = &ctx.data.read().await;
  let shard_manager = match context_data.get::<ShardManagerContainer>() {
      Some(shrd_manage) => shrd_manage,
      None => {
          eprintln!("error in ^ping | failed to get shard manager");
          return Ok(());
      }
  };

  let manager = shard_manager.lock().await;
  let runners = manager.runners.lock().await;

  let runner = match runners.get(&ShardId(ctx.shard_id)) {
      Some(runner) => runner,
      None => {
          eprintln!("error in ^ping | failed to get shard");
          return Ok(());
      }
  };

  // get latency as secs and convert it to ms
  let shard_latency = runner.latency.unwrap().as_millis();

  msg.reply(ctx, format!("pong! {}ms", shard_latency))
      .await?;

  Ok(())
}

#[group]
#[commands(ping)]
struct Ping;