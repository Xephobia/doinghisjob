use serenity::{client::bridge::gateway::ShardManager, framework::StandardFramework, http::Http, prelude::*};
use std::{collections::{HashMap, HashSet}, sync::Arc};

mod handler;
mod commands;


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
    let bot_token = std::env::var("BOTTOKEN").expect("put bot token in env var BOTTOKEN");
    let http = Http::new_with_token(&bot_token);
    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            if let Some(team) = info.team {
                owners.insert(team.owner_user_id);
            } else {
                owners.insert(info.owner.id);
            }

            match http.get_current_user().await {
                Ok(bot_id) => (owners, bot_id.id),
                Err(why) => panic!("Could not access the bot id: {:?}", why),
            }
        },
        Err(why) => panic!("Could not access application info: {:?}", why),
    };
    let bot_framework = StandardFramework::new()
    .configure(|c|{
        c.with_whitespace(true)
        .on_mention(Some(bot_id))
        .owners(owners)
        .prefix("^")
    })
    .group(&commands::ping::PING_GROUP)
    .group(&commands::crypto::CRYPTO_GROUP)
    .group(&commands::random::RANDOM_GROUP);
    
    let mut bot_client = Client::builder(&bot_token).event_handler(handler::Handler).framework(bot_framework).await.expect("failed to build client");

    {
        let mut data = bot_client.data.write().await;
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<ShardManagerContainer>(Arc::clone(&bot_client.shard_manager));
    }

    bot_client.start().await.expect("failed to start client");
}
