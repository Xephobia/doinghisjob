use serde_json::Value;
use serenity::{async_trait, model::gateway::Ready, prelude::*};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _ctx: Context, _data_about_bot: Ready) {
        println!(
            "bot is ready!\nsession id : {}\ngateway version : {}",
            _data_about_bot.session_id, _data_about_bot.version
        );
    }
    async fn unknown(&self, _ctx: Context, _name: String, _raw: Value) {
        eprintln!("recieved uknown event\nname : {}\nvalue : {}", _name, _raw);
    }
}
