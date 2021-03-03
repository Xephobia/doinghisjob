use serenity::{
    framework::standard::{
        macros::{command, group},
        CommandResult,
    },
    model::prelude::*,
    prelude::*,
};

#[command]
#[aliases("u8")]
#[description = "generates a random 8 bit unsigned number"]
async fn _u8(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<u8>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("u16")]
#[description = "generates a random 16 bit unsigned number"]
async fn _u16(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<u16>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("u32")]
#[description = "generates a random 32 bit unsigned number"]
async fn _u32(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<u32>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("u64")]
#[description = "generates a random 64 bit unsigned number"]
async fn _u64(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<u64>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("u128")]
#[description = "generates a random 128 bit unsigned number"]
async fn _u128(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<u128>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("i8")]
#[description = "generates a random 8 bit signed number"]
async fn _i8(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<i8>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("i16")]
#[description = "generates a random 16 bit signed number"]
async fn _i16(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<i16>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("i32")]
#[description = "generates a random 32 bit signed number"]
async fn _i32(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<i32>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("i64")]
#[description = "generates a random 64 bit signed number"]
async fn _i64(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<i64>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("i128")]
#[description = "generates a random 128 bit signed number"]
async fn _i128(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<i128>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("f32")]
#[description = "generates a random 32 bit float"]
async fn _f32(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<f32>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[command]
#[aliases("f64")]
#[description = "generates a random 64 bit float"]
async fn _f64(ctx: &Context, msg: &Message) -> CommandResult {
    if let Err(why) = msg.reply(ctx, rand::random::<f64>()).await {
        eprintln!(
            "error occured on {} at {}\nwhy : {}",
            std::file!(),
            std::line!(),
            why
        );
    };
    Ok(())
}

#[group]
#[prefix = "random"]
#[commands(_u8, _u16, _u32, _u64, _u128, _i8, _i16, _i32, _i64, _i128, _f32, _f64)]
struct Random;
