use serenity::client::Context;
use utils::chatterbots::cleverbot::Cleverbot;
use utils::sharekvp::CleverbotToken;

#[allow(dead_code)]
command!(think(_ctx, message, _args, _text: String) {
    let _ = message.reply(format!("You asked the (broken) kleverbot {{{}}}", _args[0..].join(" ")).as_str());
});

#[allow(dead_code)]
command!(restart(_ctx, message, _args) {
    match new_cb_session(_ctx) {
        Ok(_cbs) => {
            let _ = message.reply("\u{1f44c}");
            ()
        },
        Err(s) => {let _ = message.reply(&format!("Error: ```{:?}```", s)); ()}
    }
    
});

#[allow(dead_code)]
fn new_cb_session(_ctx: &mut  Context) -> Result<Cleverbot, String> {
    let data = _ctx.data.lock().unwrap();
    let token = data.get::<CleverbotToken>().unwrap().clone();
    Ok(Cleverbot::new(token))
}