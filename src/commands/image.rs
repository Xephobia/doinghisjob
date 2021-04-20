use std::borrow::Cow;

use image::{DynamicImage, GenericImageView, ImageOutputFormat};
use serenity::{
    framework::standard::{macros::command, CommandResult},
    http::AttachmentType,
    model::prelude::*,
    prelude::*,
};
use tokio::task::spawn_blocking;

#[command]
#[description = "add JPEG compression artifacts"]
async fn jpeg(ctx: &Context, msg: &Message) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;
    let imgs = {
        let mut v = Vec::with_capacity(msg.attachments.len());
        for a in msg.attachments.iter() {
            let i = image::load_from_memory(a.download().await?.as_slice())?;
            // build Vec to store encoded jpeg
            let mut store_vec = Vec::with_capacity(i.as_bytes().len());
            // save to jpeg
            i.write_to(&mut store_vec, ImageOutputFormat::Jpeg(10))?;
            // construct AttachmentType::Bytes needed for sending files
            let b = AttachmentType::Bytes {
                data: Cow::from(store_vec),
                filename: a.filename.clone(),
            };
            v.push(b)
        }
        v
    };
    msg.channel_id.send_files(&ctx.http, imgs, |m| m).await?;
    typing.stop().unwrap();
    Ok(())
}

#[command]
async fn magick(ctx: &Context, msg: &Message) -> CommandResult {
    let imgs = {
        let mut v = Vec::with_capacity(msg.attachments.len());
        for a in msg.attachments.iter() {
            let i = image::load_from_memory(a.download().await?.as_slice())?;
            let (width, height) = i.dimensions();
            let magick = DynamicImage::ImageRgba8(
                spawn_blocking(move || seamcarving::resize(&i, width / 2, height / 2)).await?,
            );
            let mut vf = Vec::with_capacity(magick.as_bytes().len());
            magick.write_to(&mut vf, ImageOutputFormat::Png)?;
            let b = AttachmentType::Bytes {
                data: Cow::from(vf),
                filename: a.filename.clone(),
            };
            v.push(b);
        }
        v
    };
    msg.channel_id.send_files(&ctx.http, imgs, |m| m).await?;
    Ok(())
}
