use chacha20::{
    cipher::{NewStreamCipher, StreamCipher},
    ChaCha20, Key, Nonce,
};
use clap::{
    App,
    AppSettings::{ColorNever, NoBinaryName},
    Arg,
};
use md5::{Digest, Md5};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use sha1::Sha1;
use sha2::{Sha224, Sha256, Sha384, Sha512, Sha512Trunc224, Sha512Trunc256};

enum Format {
    Hex,
    Base64,
}

macro_rules! arg_data {
    () => {
        Arg::with_name("data")
            .short("d")
            .required(true)
            .takes_value(true)
    };
}

macro_rules! arg_format {
    () => {
        Arg::with_name("format")
            .short("f")
            .default_value("base64")
            .possible_values(&["hex", "base64"])
    };
}

macro_rules! gen_hash_funcs {
    [ $($f:ident = $fhash:ident ), *] => {
        $(
            async fn $f(ctx: &Context, msg: &Message, format: Format, data: &str) -> CommandResult {
            let hash = $fhash::digest(data.as_bytes());
            let encoded = match format {
                Format::Hex => hex::encode(hash),
                Format::Base64 => base64::encode(hash),
            };
            msg.reply(ctx, encoded).await?;
            Ok(())
        } )*
        fn hash_subcmds<'a, 'b>() -> Vec<App<'a, 'b>> {
                    vec![$(App::new(stringify!($f)).args(&[arg_data!(), arg_format!()]),)*]
        }
    }
}

gen_hash_funcs![
    md5 = Md5,
    sha1 = Sha1,
    sha224 = Sha224,
    sha256 = Sha256,
    sha384 = Sha384,
    sha512 = Sha512,
    sha512_224 = Sha512Trunc224,
    sha512_256 = Sha512Trunc256
];

#[command]
#[description = "hash data with algorithm"]
async fn hash(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let matches = {
        let app = App::new("hash")
            .settings(&[NoBinaryName, ColorNever])
            .subcommands(hash_subcmds());
        app.get_matches_from_safe(args.raw_quoted())?
    };

    macro_rules! hash_subcmd_match {
        [$x:ident, $($h:ident),*] => {
            $(
                if let Some(m) = $x.subcommand_matches(stringify!($h)) {
                    let format = match m.value_of("format") {
                        Some("hex") => Format::Hex,
                        _ => Format::Base64,
                    };

                    $h(ctx, msg, format, m.value_of("data").unwrap()).await?;
                }
            )*
        };
    }

    hash_subcmd_match![matches, md5, sha1, sha224, sha256, sha384, sha512, sha512_224, sha512_256];

    Ok(())
}

#[command]
#[description = "encrypt data with algorithm"]
async fn encrypt(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let matches = {
        let app = App::new("encrypt")
            .settings(&[NoBinaryName, ColorNever])
            .subcommand(App::new("chacha20").args(&[
                arg_data!(),
                Arg::with_name("key").short("k").takes_value(true),
                Arg::with_name("nonce").short("n").takes_value(true),
                arg_format!(),
            ]));

        app.get_matches_from_safe(args.raw_quoted())?
    };

    if let Some(m) = matches.subcommand_matches("chacha20") {
        let format = match m.value_of("format") {
            Some("hex") => Format::Hex,
            _ => Format::Base64,
        };

        chacha20_enc(
            ctx,
            msg,
            format,
            m.value_of("data").unwrap(),
            m.value_of("key"),
            m.value_of("nonce"),
        )
        .await?;
    }

    Ok(())
}

#[command]
#[description = "decrypt data with algorithm"]
async fn decrypt(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let matches = {
        let app = App::new("decrypt")
            .settings(&[NoBinaryName, ColorNever])
            .subcommand(App::new("chacha20").args(&[
                arg_data!(),
                Arg::with_name("key").short("k").takes_value(true),
                Arg::with_name("nonce").short("n").takes_value(true),
                arg_format!(),
            ]));

        app.get_matches_from_safe(args.raw_quoted())?
    };

    if let Some(m) = matches.subcommand_matches("chacha20") {
        let format = match m.value_of("format") {
            Some("hex") => Format::Hex,
            _ => Format::Base64,
        };
        chacha20_dec(
            ctx,
            msg,
            format,
            m.value_of("data").unwrap(),
            m.value_of("key").unwrap(),
            m.value_of("nonce").unwrap(),
        )
        .await?;
    }

    Ok(())
}

async fn chacha20_enc(
    ctx: &Context,
    msg: &Message,
    format: Format,
    data: &str,
    key: Option<&str>,
    nonce: Option<&str>,
) -> CommandResult {
    let mut data = data.as_bytes().to_vec();
    let mut random = false;
    let (key, key_encoded): (Key, _) = match key {
        Some(k) if k.len() > 32 => return Err("key needs to be 32 bytes long or less".into()),
        Some(k) => {
            // add padding for smaller keys
            let mut padded = k.as_bytes().to_vec();
            padded.resize(32, 0);
            let encoded = match format {
                Format::Hex => hex::encode(k),
                Format::Base64 => base64::encode(k),
            };
            (Key::clone_from_slice(&padded), encoded)
        }
        None => {
            random = true;
            let k = rand::random::<[u8; 32]>();
            let encoded = match format {
                Format::Hex => hex::encode(k),
                Format::Base64 => base64::encode(k),
            };
            (k.into(), encoded)
        }
    };
    let (nonce, nonce_encoded): (Nonce, _) = match nonce {
        Some(n) if n.len() > 12 => return Err("nonce needs to be 12 bytes long or less".into()),
        Some(n) => {
            // add padding for smaller nonces
            let mut padded = n.as_bytes().to_vec();
            padded.resize(12, 0);
            let encoded = match format {
                Format::Hex => hex::encode(n),
                Format::Base64 => base64::encode(n),
            };
            (Nonce::clone_from_slice(&padded), encoded)
        }
        None => {
            random = true;
            let n = rand::random::<[u8; 12]>();
            let encoded = match format {
                Format::Hex => hex::encode(n),
                Format::Base64 => base64::encode(n),
            };
            (n.into(), encoded)
        }
    };
    let mut cipher = ChaCha20::new(&key, &nonce);
    cipher.encrypt(&mut data);
    let data_encoded = match format {
        Format::Hex => hex::encode(data),
        Format::Base64 => base64::encode(data),
    };
    if random {
        msg.author
            .direct_message(&ctx, |m| {
                m.embed(|e| {
                    e.field("key", key_encoded, true)
                        .field("nonce", nonce_encoded, true)
                        .description(format!("data :\n{}", data_encoded))
                })
            })
            .await?;
    } else {
        msg.channel_id.say(ctx, data_encoded).await?;
    }
    Ok(())
}

async fn chacha20_dec(
    ctx: &Context,
    msg: &Message,
    format: Format,
    data: &str,
    key: &str,
    nonce: &str,
) -> CommandResult {
    let (mut data, key_decoded, nonce_decoded) = match format {
        Format::Hex => (hex::decode(data)?, hex::decode(key)?, hex::decode(nonce)?),
        Format::Base64 => (
            base64::decode(data)?,
            base64::decode(key)?,
            base64::decode(nonce)?,
        ),
    };
    let key: Key = match key_decoded {
        k if k.len() > 32 => return Err("key needs to be 32 bytes long or less".into()),
        mut k => {
            // add padding for smaller keys
            k.resize(32, 0);
            Key::clone_from_slice(&k)
        }
    };
    let nonce: Nonce = match nonce_decoded {
        n if n.len() > 12 => return Err("nonce needs to be 12 bytes long or less".into()),
        mut n => {
            // add padding for smaller nonces
            n.resize(12, 0);
            Nonce::clone_from_slice(&n)
        }
    };
    let mut cipher = ChaCha20::new(
        &Key::clone_from_slice(&key),
        &Nonce::clone_from_slice(&nonce),
    );
    cipher.decrypt(&mut data);
    msg.reply(ctx, String::from_utf8(data)?).await?;
    Ok(())
}
