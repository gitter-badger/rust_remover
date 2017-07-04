use serenity::prelude::*;
use serenity::client::CACHE;
use serenity::voice;
use serenity::model::*;
use serenity::Result as SerenityResult;

struct VoiceEventHandler;

impl EventHandler for VoiceEventHandler {
    fn on_ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}



command!(join(_ctx, msg) {
    
})

command!(leave(_ctx, msg) {
    
})

command!(play(_ctx, msg, args) {

})
