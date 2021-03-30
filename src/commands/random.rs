use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
#[aliases("u8")]
#[description = "generates a random 8 bit unsigned number"]
async fn _u8(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<u8>()).await?;
    Ok(())
}

#[command]
#[aliases("u16")]
#[description = "generates a random 16 bit unsigned number"]
async fn _u16(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<u16>()).await?;
    Ok(())
}

#[command]
#[aliases("u32")]
#[description = "generates a random 32 bit unsigned number"]
async fn _u32(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<u32>()).await?;
    Ok(())
}

#[command]
#[aliases("u64")]
#[description = "generates a random 64 bit unsigned number"]
async fn _u64(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<u64>()).await?;
    Ok(())
}

#[command]
#[aliases("u128")]
#[description = "generates a random 128 bit unsigned number"]
async fn _u128(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<u128>()).await?;
    Ok(())
}

#[command]
#[aliases("i8")]
#[description = "generates a random 8 bit signed number"]
async fn _i8(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<i8>()).await?;
    Ok(())
}

#[command]
#[aliases("i16")]
#[description = "generates a random 16 bit signed number"]
async fn _i16(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<i16>()).await?;
    Ok(())
}

#[command]
#[aliases("i32")]
#[description = "generates a random 32 bit signed number"]
async fn _i32(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<i32>()).await?;
    Ok(())
}

#[command]
#[aliases("i64")]
#[description = "generates a random 64 bit signed number"]
async fn _i64(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<i64>()).await?;
    Ok(())
}

#[command]
#[aliases("i128")]
#[description = "generates a random 128 bit signed number"]
async fn _i128(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<i128>()).await?;
    Ok(())
}

#[command]
#[aliases("f32")]
#[description = "generates a random 32 bit float"]
async fn _f32(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<f32>()).await?;
    Ok(())
}

#[command]
#[aliases("f64")]
#[description = "generates a random 64 bit float"]
async fn _f64(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, rand::random::<f64>()).await?;
    Ok(())
}
