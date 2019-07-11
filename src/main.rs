use kankyo;
use sere_dc::core::framework::DvtFramework;
use serenity::client::Client;
use serenity::framework::standard::{
    macros::{command, group},
    Args, CommandResult, StandardFramework,
};
use serenity::model::{channel::Message, id::UserId};
use serenity::prelude::{Context, EventHandler};
use serenity::utils::parse_username;

use std::env;

struct Handler;

impl EventHandler for Handler {}

fn main() {
    kankyo::init();
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token").as_str(), Handler)
        .expect("Error creating client");
    client.with_framework(DvtFramework::new());

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
