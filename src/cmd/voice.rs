use serenity::prelude::*;
use serenity::client::CACHE;
use serenity::voice;
use serenity::Result as SerenityResult;
use serenity::client::Context;
use serenity::model::{Ready, ChannelId};
use utils;

/*struct VoiceEventHandler;

impl EventHandler for VoiceEventHandler {
    fn on_ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}*/



command!(join(ctx, msg, args) {
    let connect_to = match args.get(0) {
        Some(arg) => match arg.parse::<u64>() {
            Ok(id) => ChannelId(id),
            Err(_why) => {
                utils::check_msg(msg.reply("Invalid voice channel ID given"));

                return Ok(());
            },
        },
        None => {
            utils::check_msg(msg.reply("Requires a voice channel ID be given"));

            return Ok(());
        },
    };

    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            utils::check_msg(msg.channel_id.say("Groups and DMs not supported"));

            return Ok(());
        },
    };

    let mut shard = ctx.shard.lock().unwrap();
    shard.manager.join(guild_id, connect_to);

    utils::check_msg(msg.channel_id.say(&format!("Joined {}", connect_to.mention())));
});

command!(leave(ctx, msg) {
    let guild_id = match CACHE.read().unwrap().guild_channel(msg.channel_id) {
        Some(channel) => channel.read().unwrap().guild_id,
        None => {
            utils::check_msg(msg.channel_id.say("Groups and DMs not supported"));

            return Ok(());
        },
    };

    let mut shard = ctx.shard.lock().unwrap();
    let has_handler = shard.manager.get(guild_id).is_some();

    if has_handler {
        shard.manager.remove(guild_id);

        utils::check_msg(msg.channel_id.say("Left voice channel"));
    } else {
        utils::check_msg(msg.reply("Not in a voice channel"));
    }
});

command!(play(_ctx, msg, args) {

});
