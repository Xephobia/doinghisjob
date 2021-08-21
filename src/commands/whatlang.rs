use lingua::LanguageDetectorBuilder;
use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::*,
    prelude::*,
};
use tokio::task::spawn_blocking;

#[command]
#[aliases("whatlang")]
#[description = "try to detect what lang the specified text is (a)"]
#[min_args(1)]
pub async fn detectlang(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let txt: String = args
        .iter::<String>()
        .map(Result::unwrap)
        .collect::<Vec<String>>()
        .join(" ");
    // await the operation to try to reduce blocking
    let detector = LanguageDetectorBuilder::from_all_languages().build();
    let detected_langs =
        spawn_blocking(move || detector.compute_language_confidence_values(txt)).await?;
    msg.channel_id
        .send_message(&ctx, |m| {
            m.embed(|e| {
                for l in detected_langs {
                    e.field(
                        format!("{:?}", l.0),
                        format!("accuracy : {}%", l.1 * 100f64),
                        true,
                    );
                }
                e
            })
        })
        .await?;
    Ok(())
}
