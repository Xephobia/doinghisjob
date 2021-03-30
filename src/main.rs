use serenity::{
    client::bridge::gateway::ShardManager, framework::StandardFramework, http::Http, prelude::*,
};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

mod commands;
mod handler;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
struct CommandCounter;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

#[tokio::main]
async fn main() {
    let bot_token = std::env::var("BOTTOKEN").unwrap();
    let http = Http::new_with_token(&bot_token);

    // this is for getting the owner and the bot id
    let (owners, bot_id) = {
        let info = http.get_current_application_info().await.unwrap();
        let mut owners = HashSet::new();
        if let Some(team) = info.team {
            owners.insert(team.owner_user_id);
        } else {
            owners.insert(info.owner.id);
        }

        (owners, http.get_current_user().await.unwrap().id)
    };

    let bot_framework = StandardFramework::new()
        .configure(|c| {
            c.with_whitespace(true)
                .on_mention(Some(bot_id))
                .owners(owners)
                .prefix("^")
        })
        .on_dispatch_error(handler::dispatch_error_hook)
        .help(&commands::help::HELP)
        .after(handler::after_hook)
        .group(&commands::OTHERS_GROUP)
        .group(&commands::CRYPTO_GROUP)
        .group(&commands::RANDOM_GROUP)
        .group(&commands::IMAGE_GROUP);

    let mut bot_client = Client::builder(&bot_token)
        .event_handler(handler::Handler)
        .framework(bot_framework)
        .await
        .unwrap();

    // we need to insert some data, this is required for the ping cmd to work
    {
        let mut data = bot_client.data.write().await;
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&bot_client.shard_manager));
    }

    bot_client.start().await.unwrap();
}
