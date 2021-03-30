use serenity::{
  prelude::*,
  model::prelude::*,
  framework::standard::{macros::command, CommandResult},
};
use std::path::Path;

#[command]
#[description = "add JPEG compression artifacts"]
async fn jpeg(ctx: &Context, msg: &Message) -> CommandResult {
  let img = match msg.attachments.get(0) {
    Some(a) => a,
    None => return Err("a".into()),
  };

  let image_downloaded = match img.download().await {
    Ok(i) => i,
    Err(why) => return Err(why.into()),
  };

  let image = match image::load_from_memory(image_downloaded.as_slice()) {
    Ok(i) => i,
    Err(why) => return Err(why.into()),
  };

  // TODO : add saving to jpeg with compression and upload to message

  Ok(())
}