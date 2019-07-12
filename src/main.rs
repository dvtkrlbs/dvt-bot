use dvt_bot::core::framework::DvtFramework;
use dvt_bot::Handler;
use kankyo;
use serenity::client::Client;


use std::env;


fn main() {
    kankyo::init().ok();
    // Login with a bot token from the environment
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token").as_str(), Handler{})
        .expect("Error creating client");
    client.with_framework(DvtFramework::new());

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
