// File: main.rs

use dotenv::dotenv;
use serenity::http::Http;

use std::collections::HashSet;
use std::env;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::StandardFramework;
use serenity::model::gateway::Ready;

use l1276::{admin::*, general::*, hook, test::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, rdy: Ready) {
        println!("{} is connected", rdy.user.name);
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with Discord bot token in the environment.
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment.");

    let http = Http::new_with_token(&token);

    let (owners, bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owners = HashSet::new();
            owners.insert(info.owner.id);

            (owners, info.id)
        }
        Err(why) => panic!("Could not access application info: {:?}", why),
    };

    // Configure the client framework
    let framework = StandardFramework::new()
        .configure(|c| {
            c
                .prefix("!")
                .with_whitespace(true)
                .on_mention(Some(bot_id))
                .owners(owners)
        })
        .group(&GENERAL_GROUP)
        .group(&TEST_GROUP)
        .before(hook::before)
        .after(hook::after)
        // Command exception
        .unrecognised_command(hook::unrecognised_command)
        .normal_message(hook::normal_message)
        // TODO: help message
        ;
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occured while running the client: {:?}", why);
    }
}
