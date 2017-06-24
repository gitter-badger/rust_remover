// Macro Uses
#[macro_use] extern crate serenity;
#[macro_use] extern crate log;
#[macro_use] extern crate serde_derive;

// Normal Imports
extern crate log4rs;
extern crate time;
extern crate typemap;
extern crate serde;
extern crate serde_json;
extern crate crypto;
extern crate url;
// extern crate reqwests;

// Custom "Crates -> Modules"
mod cmd;
mod utils;

use serenity::model::Game;
use serenity::ext::framework::{DispatchError, help_commands};
use serenity::client::Context;
use serenity::Client;
use serenity::model::{Message, permissions};
use std::env;
use std::collections::HashMap;
use std::fmt::Write;
use utils::sharekvp::{CommandCounter, StartupTime, ReducedReadyPayload};

const CLIENT_PREFIX: &'static str = "x?";



fn main() {

    // Get token from file & login
    let mut client = Client::login(&env::var("DISCORD_TOKEN").expect("token"));
    let logfile = match env::var("RUST_REMOVER_LOG4RS") {
        Ok(file) => file,
        Err(_) => String::from("log4rs.yml")
    };
    // Init Logger
    log4rs::init_file(logfile, Default::default()).unwrap();

    info!("Booting bot...");

    debug!("Initializing command counter (what ever it is...)!");
    {
        let mut data = client.data.lock().unwrap();
        data.insert::<CommandCounter>(HashMap::default());
        data.insert::<StartupTime>(time::now());
    }
    // Contruct avalible commands
    debug!("Contructing framework...");
    client.with_framework(
        |f| f
	      // Configures the client, allowing for options to mutate how the
	      // framework functions.
	      //
	      // Refer to the documentation for
	      // `serenity::ext::framework::Configuration` for all available
	      // configurations.
	          .configure(|c| c
	                     .allow_whitespace(false)
	                     .on_mention(true)
	                     .prefix(CLIENT_PREFIX))
	      // Set a function to be called prior to each command execution. This
	      // provides the context of the command, the message that was received,
	      // and the full name of the command that will be called.
	      //
	      // You can not use this to determine whether a command should be
	      // executed. Instead, `set_check` is provided to give you this
	      // functionality.
	          .before(|ctx, msg, command_name| {
                {
	                  info!("Got command '{}' by user 'U_{}' in 'C_{} @ G_{}'",
	                        command_name,
	                        msg.author.id,
                          msg.channel_id,
                          msg.guild_id().unwrap()
                    );
                }

	              // Increment the number of times this command has been run once. If
	              // the command's name does not exist in the counter, add a default
	              // value of 0.
	              let mut data = ctx.data.lock().unwrap();
	              let counter = data.get_mut::<CommandCounter>().unwrap();
	              let entry = counter.entry(command_name.clone()).or_insert(0);
	              *entry += 1;

	              true // if `before` returns false, command processing doesn't happen.
	          })
	      // Similar to `before`, except will be called directly _after_
	      // command execution.
	          .after(|_, _, command_name, error| {
	              match error {
	                  Ok(()) => debug!("Processed command '{}'", command_name),
	                  Err(why) => info!("Command '{}' returned error {:?}", command_name, why),
	              }
	          })
	      // Set a function that's called whenever a command's execution didn't complete for one
	      // reason or another. For example, when a user has exceeded a rate-limit or a command
	      // can only be performed by the bot owner.
	          .on_dispatch_error(|_ctx, msg, error| {
	              match error {
	                  DispatchError::RateLimited(seconds) => {
	                      let _ = msg.channel_id.say(&format!("Try this again in {} seconds.", seconds));
	                  },
	                  // Any other error would be silently ignored.
	                  _ => {},
		  	        }
		        })
            .simple_bucket("short", 2)
            .simple_bucket("extended", 5)
            .bucket("complicated", 5, 30, 2)
            .group("Utility", |g|
                   g.command("about", |c|
                             c.exec_str("Small bot written using rust")
                             .bucket("extended")
                             .desc("Info about why: More under stats & info"))
                   .command("commands",|c|
                            c.bucket("complicated")
                            .exec(commands)
                            .desc("How much was an command used")
                            .required_permissions(permissions::ADMINISTRATOR)
                            .check(owner_check))
                   .command("stats", |c| c.bucket("complicated").exec(cmd::info::status).desc("Gives Status infos about the bot"))
                   .command("help", |c| c.exec_help(help_commands::with_embeds).bucket("short"))
                   .command("embed", |c|
                            c.exec(cmd::utils::embed)
                            .check(owner_check)
                            .desc("Create an Custom embed. OWNER ONLY"))
                   .command("purge_self", |c|
                            c.exec(cmd::utils::purge_self)
                            .check(owner_check)
                            .desc("Purges x bot messages"))
                   .command("purge", |c|
                            c.exec(cmd::utils::purge)
                            .check(owner_check)
                            .desc("Purges x messages")))
            .group("Cleverbot", |g|
                   g.command("think", |c|
                             c.bucket("extended")
                             .exec(cmd::cleverbot::think)
                             .known_as("ask")
                             .known_as("cleverbot")
                             .known_as("cb")
                             .check(owner_check)
                             .desc("Ask Cleverbot (if the API isnt Broken)"))
                   .command("cbrestart", |c|
                            c.bucket("extended")
                            .exec(cmd::cleverbot::restart)
                            .desc("Reinitialize the Cleverbot Session")
                            .check(owner_check)))
    );

    // Startup
    debug!("Creating callbacks...");
    // Create on_ready callback & game setting
    client.on_ready(|_context, ready| {
        //println!("{} is connected", ready.user.name);
        info!("Bot booted & connected as {}", ready.user.name);
        _context.set_game(Game::playing("x? | Removing Rust"));

        // Setting Data
        {
            let mut data = _context.data.lock().unwrap();
            data.insert::<ReducedReadyPayload>(ReducedReadyPayload {
                session_id: ready.session_id,
                shard: ready.shard,
                version: ready.version
            });
        }
    });
    info!("Attempting to start Shards....");
    // start listening for events by starting a single shard
    let v = client.start();

    println!("Exited...\nCode Recieved:\n{:?}", v);

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
#[allow(unused_variables)]
command!(say(_context, message, _args, text: String) {
    if _args[0] == ":embed" {
        let _ = _context.channel_id.unwrap().send_message(
            |m| m.content(" ").embed(
                |e| e.description(_args[1..].join(" ").as_ref())
            )
        );
    } else {
        let _ = _context.channel_id.unwrap().send_message(
            |m| m.content(_args.join(" ").as_ref())
        );
    }
});

/*
TODO

commands:
-> info
--> Bot info (version, name etc)

-> stats » statistics
--> Statistics about guilds served & member count.
--> https://github.com/acdenisSK/kitty/blob/master/commands/stats.go

-> sinfo
--> Info about the server the bot is residing on.
--> https://github.com/acdenisSK/kitty/blob/master/commands/sinfo.go

-> eval (LOW PRIORITY)
--> Evaluate rust-expressions

-> embed
--> pure embed command (move embed out of say) // DONE (Let in say to keep Compatibility)

 */
