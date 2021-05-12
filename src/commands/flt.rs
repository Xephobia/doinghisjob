use num::{rational::Ratio, Float};
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
pub async fn flt(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let f: f64 = args.parse()?;
    let (m, e, s) = f.integer_decode();
    let format_exp = e.to_string() + &format!("\n`{:b}`", e); // the exponent is only 11 bits
    let format_mant = m.to_string() + &format!("\n`{:b}`", e); // the mantissa is only 52 bits
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                e.field("float", f, false)
                    .field(
                        "ratio",
                        Ratio::from_float(f).map_or("None".to_string(), |r| r.to_string()),
                        false,
                    )
                    .field("sign", s, false)
                    .field("exponent", format_exp, false)
                    .field("mantissa", format_mant, false)
            })
        })
        .await?;

    Ok(())
}
