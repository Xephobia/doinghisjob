use num::rational::Ratio;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};

#[command]
pub async fn flt(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let f: f64 = args.parse()?;
    msg.channel_id
        .send_message(&ctx.http, |m| {
            m.embed(|e| {
                let e = e.field("float", f, false).field(
                    "ratio",
                    Ratio::from_float(f).map_or("irrational".to_string(), |r| r.to_string()),
                    false,
                );
                let category = match f.classify() {
                    std::num::FpCategory::Nan => "NaN",
                    std::num::FpCategory::Infinite => "infinite",
                    std::num::FpCategory::Zero => "zero",
                    std::num::FpCategory::Subnormal => "subnormal",
                    std::num::FpCategory::Normal => "normal",
                };
                e.field("type", category, false)
            })
        })
        .await?;

    Ok(())
}
