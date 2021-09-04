use paste::paste;
use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::*,
    prelude::*,
};

macro_rules! gen_random_funs {
    [$($x:ty),*] => {
        paste! {
        $(
            #[command]
            #[aliases($x)]
            #[description = "generates a random `" $x "`"]
            async fn [<_ $x>](ctx: &Context, msg: &Message) -> CommandResult {
                msg.reply(ctx, rand::random::<$x>()).await?;
                Ok(())
            }

        )*
    }
    };
}

gen_random_funs![u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64];
