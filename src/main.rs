#![deny(deprecated)]

// Macro Uses
#[macro_use] extern crate serenity;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

// Normal Imports
extern crate log4rs;
extern crate typemap;
extern crate serde;
extern crate serde_json;
extern crate crypto;
extern crate chrono;
extern crate rand;
extern crate rayon;

#[cfg(feature="memory-stats")]
extern crate psutil;

#[cfg(feature = "cleverbot")]
extern crate cleverbot_api;

// Custom "Crates -> Modules"
mod cmd;
mod utils;
mod statics;

use serenity::model::Game;
use serenity::ext::framework::{DispatchError, help_commands};
use serenity::client::Context;
use serenity::Client;
use serenity::model::{Message, permissions};
use serenity::ext::framework::Framework;
use std::env;
use std::collections::HashMap;
use std::fmt::Write;
use utils::sharekvp::{CommandCounter, StartupTime};
#[cfg(feature = "cleverbot")]
use utils::sharekvp::CleverbotToken;
use chrono::prelude::Local;

const CLIENT_PREFIX: &'static str = "x?";



fn main() {

    // Get token from file & login
    let logfile = match env::var("RUST_REMOVER_LOG4RS") {
        Ok(file) => file,
        Err(_) => String::from("log4rs.yml")
    };
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("DISCORD_TOKEN"));
    #[cfg(feature = "cleverbot")]
    let cleverbot_token = env::var("CLEVERBOT_TOKEN").expect("CLEVERBOT_TOKEN");
    // Init Logger
    log4rs::init_file(logfile, Default::default()).unwrap();

    info!("Booting bot...");

    debug!("Initializing command counter !");
    {
        let mut data = client.data.lock().unwrap();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<StartupTime>(Local::now());
        #[cfg(feature = "cleverbot")]
        data.insert::<CleverbotToken>(String::from(cleverbot_token));
    }
    // Contruct avalible commands
    debug!("Contructing framework...");
    client.with_framework(build_framework);

    // Startup
    debug!("Creating callbacks...");
    // Create on_ready callback & game setting
    client.on_ready(|_context, ready| {
        _context.set_game(Game::playing("x? | Removing Rust"));
        if let Some(shards) = ready.shard {
            info!("Started as {}#{} on Shard {}/{} with {} Guilds. [SERENITY VERSION: {}]", ready.user.name, ready.user.discriminator, shards[0], shards[1], ready.guilds.len(), ready.version);
        } else {
            info!("Started as {}#{} on Shard NaN/NaN with {} Guilds. [SERENITY VERSION: {}]", ready.user.name, ready.user.discriminator, ready.guilds.len(), ready.version);
        }
    });
    info!("Attempting to start Shards....");
    // start listening for events by starting a single shard
    let v = client.start();

    error!("Client Crash !\nError Code:\n{:#?}", v);

}

fn build_framework(f: Framework) -> Framework {
    
    let mut f = f.configure(|c| c.allow_whitespace(false).on_mention(true).prefix(CLIENT_PREFIX));
    f = f.before(|ctx, msg, command_name| {
        {
            info!("Got command '{}' by user 'U_{}' in 'C_{} @ G_{}'",
                command_name,
                msg.author.id,
                msg.channel_id,
                msg.guild_id().unwrap()
            );
        }
        let mut data = ctx.data.lock().unwrap();
        let counter = data.get_mut::<CommandCounter>().unwrap();
        let entry = counter.entry(command_name.clone()).or_insert(0);
        *entry += 1;

        true // if `before` returns false, command processing doesn't happen.
    });

	f = f.after(|_, _, command_name, error| {
        match error {
            Ok(()) => debug!("Processed command '{}'", command_name),
            Err(why) => info!("Command '{}' returned error {:?}", command_name, why),
        }
    });
    f = f.on_dispatch_error(|_ctx, msg, error| {
        match error {
            DispatchError::RateLimited(seconds) => {
                let _ = msg.channel_id.say(&format!("Try this again in {} seconds.", seconds));
            },
            DispatchError::LackOfPermissions(permissions) => {
                warn!("Lacking Permissions ({:?}) for '{}' in '{}'", permissions, msg.content, msg.channel_id);
            },
            _ => {},
        }
    });
    // Buckets
    f = f.simple_bucket("short", 2).simple_bucket("extended", 5).bucket("complicated", 5, 30, 2);

    // Utility Commands
    f = f.group("Utility", |g| g
        .command("about", |c| c
            .exec_str("A small bot i have written for myself. its for fun so dont expect anything.\n\n**Author:** HeapUnderflow#9358")
            .bucket("extended")
            .desc("About the Bot"))
        .command("commands",|c| c
            .bucket("complicated")
            .exec(commands)
            .desc("How much was an command used\n**REQUIRED**: Bot Owner")
            .required_permissions(permissions::ADMINISTRATOR)
            .check(owner_check))
        .command("stats", |c| c.bucket("complicated").exec(cmd::info::status).desc("Gives Status infos about the bot"))
        .command("help", |c| c.exec_help(help_commands::with_embeds).bucket("short"))
        .command("embed", |c| c
            .exec(cmd::utilcmd::embed)
            .check(owner_check)
            .desc("Create an Custom embed.\n**REQUIRED**: Bot Owner"))
        .command("purge_self", |c| c
            .exec(cmd::utilcmd::purge_self)
            .check(owner_check)
            .desc("Purges x bot messages\n**REQUIRED**: Bot Owner"))
        .command("purge", |c| c
            .exec(cmd::utilcmd::purge)
            .check(owner_check)
            .desc("Purges x messages\n**REQUIRED**: Bot Owner"))
        .command("guild_info", |c| c
            .exec(cmd::info::guild_info)
            .check(owner_check)
            .desc("Retrieves info about the Guild\n**REQUIRED**: Bot Owner")
            .known_as("guildinfp")
            .known_as("iguild")));
    #[cfg(feature = "cleverbot")]
    {
        // Cleverbot Commands
        f = f.group("Cleverbot", |g| g
                .command("think", |c| c
                    .bucket("extended")
                    .exec(cmd::cleverbot::think)
                    .known_as("ask")
                    .known_as("cleverbot")
                    .known_as("cb")
                    .check(owner_check)
                    .desc("Ask Cleverbot (if the API isnt Broken)\n**REQUIRED**: Bot Owner"))
                .command("cbrestart", |c| c
                    .bucket("extended")
                    .exec(cmd::cleverbot::restart)
                    .desc("Reinitialize the Cleverbot Session\n**REQUIRED**: Bot Owner")
                    .check(owner_check)));
    }
    
    // Misc Commands
    f = f.group("Misc", |g| g
            .command("twitch", |c| c
                .bucket("simple")
                .exec_str("Twitch is probably broken again.\nBut should it work, theres an account you could check out: https://twitch.tv/the__cj")
                .desc("About Twitch")));

    f = f.group("Reminders", |g| g
            .command("remind", |c| c
                .bucket("extended")
                .exec(cmd::remind::add_reminder)
                .check(owner_check)
                .desc("Add an reminder for an duration\n**REQUIRED**: Bot Owner")));
    f
}


#[allow(dead_code)]
fn owner_check(_: &mut Context, message: &Message) -> bool {
    message.author.id == 102379663615094784
}


command!(commands(ctx, msg, _args) {
    let mut contents = "Commands used:\n".to_owned();

    let data = ctx.data.lock().unwrap();
    let counter = data.get::<CommandCounter>().unwrap();

    for (k, v) in counter {
        let _ = write!(contents, "- {name}: {amount}\n", name=k, amount=v);
    }

    if let Err(why) = msg.channel_id.say(&contents) {
        println!("Error sending message: {:?}", why);
    }
});



command!(ping(_context, message) {
    let _ = message.reply("Pong!");
    info!("Answered {{{}}} by {}#{} ({})", &message.content[2..], message.author.name, message.author.discriminator, message.author.id);
});