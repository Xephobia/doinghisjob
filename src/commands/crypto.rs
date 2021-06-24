use anyhow::anyhow;
use chacha20::{
    cipher::{NewStreamCipher, StreamCipher},
    ChaCha20, Key, Nonce,
};
use hex::{decode, encode};
use md5::Digest;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[command]
#[description = "hashes data with md5 algorithm"]
#[usage = "data"]
#[min_args(1)]
async fn md5(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = args.rest();
    let hash = md5::Md5::digest(data.as_bytes());
    msg.reply(ctx, format!("{:x}", hash)).await?;
    Ok(())
}

#[command]
#[description = "hashes data with sha1 algorithm"]
#[usage = "data"]
#[min_args(1)]
async fn sha1(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = args.rest();
    let hash = sha1::Sha1::digest(data.as_bytes());
    msg.reply(ctx, format!("{:x}", hash)).await?;
    Ok(())
}

#[command]
#[description = "hashes data with sha2-224 algorithm"]
#[usage = "data"]
#[min_args(1)]
async fn sha224(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = args.rest();
    let hash = sha2::Sha224::digest(data.as_bytes());
    msg.reply(ctx, format!("{:x}", hash)).await?;
    Ok(())
}

#[command]
#[description = "hashes data with sha2-256 algorithm"]
#[usage = "data"]
#[min_args(1)]
async fn sha256(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = args.rest();
    let hash = sha2::Sha256::digest(data.as_bytes());
    msg.reply(ctx, format!("{:x}", hash)).await?;
    Ok(())
}

#[command]
#[description = "hashes data with sha2-512 algorithm truncated to 384 bits"]
#[usage = "data"]
#[min_args(1)]
async fn sha384(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = args.rest();
    let hash = sha2::Sha384::digest(data.as_bytes());
    msg.reply(ctx, format!("{:x}", hash)).await?;
    Ok(())
}

#[command]
#[description = "hashes data with sha2-512 algorithm"]
#[usage = "data"]
#[min_args(1)]
async fn sha512(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = args.rest();
    let hash = sha2::Sha512::digest(data.as_bytes());
    msg.reply(ctx, format!("{:x}", hash)).await?;
    Ok(())
}

#[command]
#[description = "hashes data with sha2-512 algorithm truncated to 224 bits"]
#[usage = "data"]
#[min_args(1)]
async fn sha512_224(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = args.rest();
    let hash = sha2::Sha512Trunc224::digest(data.as_bytes());
    msg.reply(ctx, format!("{:x}", hash)).await?;
    Ok(())
}

#[command]
#[description = "hashes data with sha2-512 algorithm truncated to 256 bits"]
#[usage = "data"]
#[min_args(1)]
async fn sha512_256(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let data = args.rest();
    let hash = sha2::Sha512Trunc256::digest(data.as_bytes());
    msg.reply(ctx, format!("{:x}", hash)).await?;
    Ok(())
}

#[command]
#[description = "hashes data with unspecified hash algorithm (for now the algorithm is SipHash 1-3, see https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html)"]
#[usage = "data"]
#[min_args(1)]
async fn hash(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut hasher = DefaultHasher::new();
    let data = args.rest();
    data.hash(&mut hasher);
    msg.reply(ctx, format!("{:x}", hasher.finish())).await?;
    Ok(())
}

#[command]
#[description = "encrypt data using ChaCha20 *note : it's better to use this command in pm*"]
#[usage = "key nonce data"]
#[min_args(3)]
async fn chacha20_enc(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args_raw = args.raw();
    // generate the key if "rand" else, get the key and check if right size
    let key: Key = match args_raw.next() {
        Some("rand") => Key::clone_from_slice(&rand::random::<[u8; 32]>()),
        Some(k) => {
            if k.len() == 32 {
                Key::clone_from_slice(k.as_bytes())
            } else {
                return Err(anyhow!("key needs to be 32 bytes").into());
            }
        }
        None => {
            return Err(anyhow!("key needs to be 32 bytes").into());
        }
    };
    // generate the nonce if "rand" else, get the nonce and check if right size
    let nonce: Nonce = match args_raw.next() {
        Some("rand") => Nonce::clone_from_slice(&rand::random::<[u8; 12]>()),
        Some(n) => {
            if n.len() == 12 {
                Nonce::clone_from_slice(n.as_bytes())
            } else {
                return Err(anyhow!("nonce needs to be 12 bytes").into());
            }
        }
        None => return Err(anyhow!("input a nonce").into()),
    };
    // will join the remaining args into a String
    let mut data = args_raw.collect::<Vec<&str>>().join(" ");

    let mut cipher = ChaCha20::new(&key, &nonce);
    cipher.encrypt(unsafe { data.as_bytes_mut() });
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.description(encode(data))
                    .field("key", format!("{:x}", key), true)
                    .field("nonce", format!("{:x}", nonce), true)
            })
        })
        .await?;
    Ok(())
}

#[command]
#[description = "decrypt data using ChaCha20 *note : it's better to use this command in pm*"]
#[usage = "key nonce data"]
#[min_args(3)]
async fn chacha20_dec(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut args_raw = args.raw();
    // get the key if right size
    let key: Key = match args_raw.next() {
        Some(k) => {
            let hex_key = match decode(k) {
                Ok(result) => result,
                Err(why) => return Err(anyhow!("key must be in valid hex format {:?}", why).into()),
            };
            if hex_key.len() == 32 {
                Key::clone_from_slice(&hex_key)
            } else {
                return Err(anyhow!("key needs to be 32 bytes").into());
            }
        }
        None => return Err(anyhow!("input a key").into()),
    };
    // get the nonce if right size
    let nonce: Nonce = match args_raw.next() {
        Some(n) => {
            let hex_nonce = match decode(n) {
                Ok(result) => result,
                Err(why) => {
                    return Err(anyhow!("nonce must be in valid hex format {:?}", why).into())
                }
            };
            if hex_nonce.len() == 12 {
                Nonce::clone_from_slice(&hex_nonce)
            } else {
                return Err(anyhow!("nonce needs to be 12 bytes").into());
            }
        }
        None => return Err(anyhow!("input a nonce").into()),
    };
    let mut data = match args_raw.next() {
        Some(d) => match decode(d) {
            Ok(result) => result,
            Err(why) => return Err(anyhow!("data must be in valid hex format {:?}", why).into()),
        },
        None => return Err(anyhow!("input data").into()),
    };
    let mut cipher = ChaCha20::new(&key, &nonce);
    cipher.decrypt(&mut data);
    msg.reply(ctx, String::from_utf8(data)?).await?;
    Ok(())
}
