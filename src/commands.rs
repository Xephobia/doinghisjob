mod crypto;
mod flt;
pub mod help;
mod image;
mod ping;
mod random;
mod whatlang;

use self::image::*;
use self::whatlang::*;
use crypto::*;
use flt::*;
use ping::*;
use random::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(
    hash, encrypt, decrypt
)]
struct Crypto;

#[group]
#[commands(latency, detectlang, flt)]
struct Others;

#[group]
#[prefix = "random"]
#[commands(_u8, _u16, _u32, _u64, _u128, _i8, _i16, _i32, _i64, _i128, _f32, _f64)]
struct Random;

#[group]
#[commands(jpeg, magick)]
struct Image;
