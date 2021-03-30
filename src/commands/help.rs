use std::collections::HashSet;

use serenity::{
    client::Context,
    framework::standard::{
        help_commands,
        macros::help,
        Args, CommandGroup, CommandResult, HelpOptions,
    },
    model::{channel::Message, id::UserId},
};

#[help]
#[no_help_available_text = "no help aviable for `{}`"]
#[command_not_found_text = "Could not find: `{}`"]
async fn help(
    context: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = help_commands::with_embeds(context, msg, args, help_options, groups, owners).await;
    Ok(())
}
