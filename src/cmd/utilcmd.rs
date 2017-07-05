use utils::linear_parse::JsonToDiscordEmbedObject;
use serenity::client::CACHE;
use serenity::model::{Message, MessageId, UserId};
use serenity::client::Context;
use chrono::Local;
use utils;

command!(embed(_ctx, msg, _args, _text: String) {
    let s: String = _args.join(" ");
    let jsprs = JsonToDiscordEmbedObject::new();
    match jsprs.parse(s.as_str()) {
        Ok(r) => utils::check_msg(msg.channel_id.send_message(|m| m.content(" ").embed(|_| r))),
        Err(e) => utils::check_msg(msg.channel_id.send_message(|m| m.content(format!("**ERROR**\n```{}```", e).as_str())))
    };
    let _ = msg.delete();
});

command!(purge_self(_ctx, msg, _args, amount: u64) {
    let count: u64 = match amount {
        1...100 => amount,
        _ => 0
    };
    let userid = CACHE.read().unwrap().user.id;
    purge_messages(_ctx, msg, count, Some(userid));
});

command!(purge(_ctx, msg, _args, amount: u64) {
    let count: u64 = match amount {
        1...100 => amount,
        _ => 0
    };
    purge_messages(_ctx, msg, count, None);
});

fn purge_messages(_ctx: &mut Context, msg: &Message, amount: u64, user: Option<UserId>) {
    if let Ok(x) = msg.channel_id.messages(|g| g.limit(100)) {
        let filtered_result: Vec<MessageId> = x.into_iter().filter(|p| {
            if let Some(uid) = user {
                if p.author.id != uid {
                    return false;
                }
            }
            let ptime = p.timestamp;
            let ctime = Local::now();
            let delta = ctime.signed_duration_since(ptime);
            if delta.num_days() > 11 {
                return false;
            }
            true

        }).map(|i| i.id).take((amount + 1) as usize).collect();
        if filtered_result.len() > 1 {
            match msg.channel_id.delete_messages(&filtered_result) {
                Ok(_) => {
                    debug!("Purge of {} OK !", msg.channel_id);
                    let _ = msg.delete();
                },
                Err(e) => warn!("Purge of {} ERR !\nARG (LENGTH: {}):  {:?}\nERR: {:?}", msg.channel_id, filtered_result.len(), filtered_result, e)
            }
        } else if filtered_result.len() == 1 {
            match msg.channel_id.delete_message(filtered_result[0]) {
                Ok(_) => {
                    debug!("Purge of {} OK !", msg.channel_id);
                    let _ = msg.delete();
                },
                Err(e) => warn!("Purge of {} ERR !\nARG (LENGTH: {}):  {:?}\nERR: {:?}", msg.channel_id, filtered_result.len(), filtered_result, e)
            }
        }
    }
}
