mod crypto;
pub mod help;
mod image;
mod ping;
mod random;
mod whatlang;

use self::image::*;
use self::whatlang::*;
use crypto::*;
use ping::*;
use random::*;
use serenity::framework::standard::macros::group;

#[group]
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

#[group]
#[commands(latency, detectlang)]
struct Others;

#[group]
#[prefix = "random"]
#[commands(_u8, _u16, _u32, _u64, _u128, _i8, _i16, _i32, _i64, _i128, _f32, _f64)]
struct Random;

#[group]
#[commands(jpeg, magick)]
struct Image;
