use serenity::client::Context;
use utils::chatterbots::cleverbot::Cleverbot;

command!(think(_ctx, message, _args, text: String) {
    let _ = message.reply(format!("You asked the (broken) kleverbot {{{}}}", _args[0..].join(" ")).as_str());
});

command!(restart(_ctx, message, _args) {
    match new_cb_session(&_ctx) {
        Ok(cbs) => {
            let _ = message.reply("\u{1f44c}");
            ()
        },
        Err(s) => {let _ = message.reply(""); ()}
    }
    
});

fn new_cb_session<'cntxt> (_ctx: &'cntxt Context) -> Result<Cleverbot, String> {
    Ok(Cleverbot::new("This Token".to_owned()))
}