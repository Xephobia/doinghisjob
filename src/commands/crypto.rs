use chacha20::{
    cipher::{NewStreamCipher, StreamCipher},
    ChaCha20, Key, Nonce,
};
use hex::{decode, encode};
use md5::Digest;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::prelude::*,
    prelude::*,
};

#[command]
#[description = "hashes content with md5 algorithm"]
#[usage = "`md5` *data*"]
async fn md5(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    if args.len() != 1 {
        msg.reply(ctx, "usage : `md5` *data*").await?;
        return Ok(());
    }

    let hash = md5::Md5::digest(args.parse::<String>()?.as_bytes());

    msg.reply(ctx, format!("{:x}", hash)).await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "hashes content with sha1 algorithm"]
#[usage = "`sha1` *data*"]
async fn sha1(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    if args.len() != 1 {
        msg.reply(ctx, "usage : `sha1` *data*").await?;
        return Ok(());
    }

    let hash = sha1::Sha1::digest(args.parse::<String>().unwrap().as_bytes());

    msg.reply(ctx, format!("{:x}", hash)).await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "hashes content with sha2-224 algorithm"]
#[usage = "`sha224` *data*"]
async fn sha224(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    if args.len() != 1 {
        msg.reply(ctx, "usage : `sha224` *data*").await?;
        return Ok(());
    }

    let hash = sha2::Sha224::digest(args.parse::<String>().unwrap().as_bytes());

    msg.reply(ctx, format!("{:x}", hash)).await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "hashes content with sha2-256 algorithm"]
#[usage = "`sha256` *data*"]
async fn sha256(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    if args.len() != 1 {
        msg.reply(ctx, "usage : `sha256` *data*").await?;
        return Ok(());
    }

    let hash = sha2::Sha256::digest(args.parse::<String>().unwrap().as_bytes());

    msg.reply(ctx, format!("{:x}", hash)).await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "hashes content with sha2-512 algorithm truncated to 384 bits"]
#[usage = "`sha384` *data*"]
async fn sha384(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    if args.len() != 1 {
        msg.reply(ctx, "usage : `sha384` *data*").await?;
        return Ok(());
    }

    let hash = sha2::Sha384::digest(args.parse::<String>().unwrap().as_bytes());

    msg.reply(ctx, format!("{:x}", hash)).await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "hashes content with sha2-512 algorithm"]
#[usage = "`sha512` *data*"]
async fn sha512(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    if args.len() != 1 {
        msg.reply(ctx, "usage : `sha512` *data*").await?;
        return Ok(());
    }

    let hash = sha2::Sha512::digest(args.parse::<String>().unwrap().as_bytes());

    msg.reply(ctx, format!("{:x}", hash)).await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "hashes content with sha2-512 algorithm truncated to 224 bits"]
#[usage = "`sha512_224` *data*"]
async fn sha512_224(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    if args.len() != 1 {
        msg.reply(ctx, "usage : `sha512_224` *data*").await?;
        return Ok(());
    }

    let hash = sha2::Sha512Trunc224::digest(args.parse::<String>().unwrap().as_bytes());

    msg.reply(ctx, format!("{:x}", hash)).await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "hashes content with sha2-512 algorithm truncated to 256 bits"]
#[usage = "`sha512_256` *data*"]
async fn sha512_256(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    if args.len() != 1 {
        msg.reply(ctx, "usage : `sha512_224` *data*").await?;
        return Ok(());
    }

    let hash = sha2::Sha512Trunc256::digest(args.parse::<String>().unwrap().as_bytes());

    msg.reply(ctx, format!("{:x}", hash)).await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "encrypt data using ChaCha20 *note : it's better to use this command in pm*"]
#[usage = "`chacha20_enc` *key* *nonce* *data*"]
async fn chacha20_enc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    let collect_args = args.raw().collect::<Vec<_>>();

    let key: Key = match collect_args.get(0) {
        Some(&"rand") => Key::clone_from_slice(&rand::random::<[u8; 32]>()),
        Some(k) => {
            if k.len() == 32 {
                Key::clone_from_slice(k.as_bytes())
            } else {
                msg.reply(ctx, "key needs to be 32 bytes").await?;
                return Ok(());
            }
        }
        None => {
            msg.reply(ctx, "input a key").await?;
            return Ok(());
        }
    };

    let nonce: Nonce = match collect_args.get(1) {
        Some(&"rand") => Nonce::clone_from_slice(&rand::random::<[u8; 12]>()),
        Some(n) => {
            if n.len() == 12 {
                Nonce::clone_from_slice(n.as_bytes())
            } else {
                msg.reply(ctx, "nonce needs to be 12 bytes").await?;
                return Ok(());
            }
        }
        None => {
            msg.reply(ctx, "input a nonce").await?;
            return Ok(());
        }
    };

    let mut data = match collect_args.get(2) {
        Some(d) => d.as_bytes(),
        None => {
            msg.reply(ctx, "input data").await?;
            return Ok(());
        }
    }
    .to_owned();

    let mut cipher = ChaCha20::new(&key, &nonce);

    cipher.encrypt(&mut data);

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description(encode(data))
                    .field("key", format!("{:x}", key), true)
                    .field("nonce", format!("{:x}", nonce), true)
            })
        })
        .await?;

    typing.stop();

    Ok(())
}

#[command]
#[description = "decrypt data using ChaCha20 *note : it's better to use this command in pm*"]
#[usage = "`chacha20_dec` *key* *nonce* *message*"]
async fn chacha20_dec(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let typing = msg.channel_id.start_typing(&ctx.http)?;

    let collect_args = args.raw().collect::<Vec<_>>();

    let key: Key = match collect_args.get(0) {
        Some(k) => {
            let hex_key = match decode(k) {
                Ok(result) => result,
                Err(_) => {
                    msg.reply(ctx, "key must be in valid hex format").await?;
                    return Ok(());
                }
            };
            if hex_key.len() == 32 {
                Key::clone_from_slice(&hex_key)
            } else {
                msg.reply(ctx, "key needs to be 32 bytes").await?;
                return Ok(());
            }
        }
        None => {
            msg.reply(ctx, "input a key").await?;
            return Ok(());
        }
    };

    let nonce: Nonce = match collect_args.get(1) {
        Some(n) => {
            let hex_nonce = match decode(n) {
                Ok(result) => result,
                Err(_) => {
                    msg.reply(ctx, "nonce must be in valid hex format").await?;
                    return Ok(());
                }
            };
            if hex_nonce.len() == 12 {
                Nonce::clone_from_slice(&hex_nonce)
            } else {
                msg.reply(ctx, "nonce needs to be 12 bytes").await?;
                return Ok(());
            }
        }
        None => {
            msg.reply(ctx, "input a nonce").await?;
            return Ok(());
        }
    };

    let mut data = match collect_args.get(2) {
        Some(d) => match decode(d) {
            Ok(result) => result,
            Err(_) => {
                msg.reply(ctx, "data must be in valid hex format").await?;
                return Ok(());
            }
        },
        None => {
            msg.reply(ctx, "input data").await?;
            return Ok(());
        }
    }
    .to_owned();

    let mut cipher = ChaCha20::new(&key, &nonce);

    cipher.decrypt(&mut data);

    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description(String::from_utf8(data).unwrap())
                    .field("key", format!("{:x}", key), true)
                    .field("nonce", format!("{:x}", nonce), true)
            })
        })
        .await?;

    typing.stop();

    Ok(())
}

#[group]
#[prefix = "crypto"]
#[commands(
    md5,
    sha1,
    sha224,
    sha256,
    sha384,
    sha512,
    sha512_224,
    sha512_256,
    chacha20_enc,
    chacha20_dec
)]
struct Crypto;
