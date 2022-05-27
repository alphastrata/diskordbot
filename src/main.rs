//
//╭━━━┳━━━┳━━━┳━━━┳━━━┳━╮╱╭┳╮╱╱╱╱╭╮
//┃╭━╮┃╭━━┫╭━╮┃╭━╮┃╭━╮┃┃╰╮┃┃┃╱╱╱╭╯╰╮
//┃┃╱╰┫╰━━┫╰━╯┃┃╱╰┫┃╱┃┃╭╮╰╯┃╰━┳━┻╮╭╯
//┃┃╭━┫╭━━┫╭━━┫┃╭━┫╰━╯┃┃╰╮┃┃╭╮┃╭╮┃┃
//┃╰┻━┃┃╱╱┃┃╱╱┃╰┻━┃╭━╮┃┃╱┃┃┃╰╯┃╰╯┃╰╮
//╰━━━┻╯╱╱╰╯╱╱╰━━━┻╯╱╰┻╯╱╰━┻━━┻━━┻━╯
//
const GFPGAN_BOT_ID: u64 = 889476441253761044; // These are harmless and unique, there is no danger in making them public.
const MAXIMUM_INPUT_RESOLUTION: u64 = 2560;

const GFPGAN_PATH: &str = "./GFPGAN/";
const ESRGAN_PATH: &str = "./ESRGAN/"; //NOTE: not in use

mod gans;
mod utils;

use chrono::Utc;
use std::env;

struct Handler;

#[allow(unused_imports)]
use serenity::{
    async_trait,
    http::client::Http,
    model::interactions::{
        application_command::{
            ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
            ApplicationCommandOptionType,
        },
        Interaction, InteractionResponseType,
    },
    model::{channel::Message, gateway::Ready, id::ChannelId},
    prelude::*,
    utils::MessageBuilder,
};

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, context: Context, message: Message) {
        if !&message.content.contains(&GFPGAN_BOT_ID.to_string()) {
            println!("{} Skipping, no work required.", Utc::now(),);
            return;
        }

        let _ = utils::process_downloadables(
            &message,
            &context,
            &GFPGAN_BOT_ID,
            MAXIMUM_INPUT_RESOLUTION,
            GFPGAN_PATH,
            ESRGAN_PATH,
        )
        .await;

        // allow remote termination...
        let _ = utils::remote_kill_triggered(&message, &GFPGAN_BOT_ID, &context).await;
    }
    // Let users know we're in the channel and ready for business
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} {} IS CONNECTED!", Utc::now(), ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let key = "DISCORD_TOKEN";
    // read the bot's token from client_secret.txt (excluded in .gitignore)
    let client_secret_path = "client_secret.txt";
    let token =
        utils::read_token_txt(client_secret_path.trim().to_string()).expect("Unable to read token");
    env::set_var(key, token);
    let intents = GatewayIntents::DIRECT_MESSAGES | GatewayIntents::GUILD_MESSAGES;
    //| GatewayIntents::MESSAGE_CONTENT;
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut _worklist: Vec<String>;
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .application_id(889476441253761044)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
