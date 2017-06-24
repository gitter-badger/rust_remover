use utils::linear_parse::JsonToDiscordEmbedObject;
use serenity::client::CACHE;
use serenity::model::{Message, MessageId, UserId};
use serenity::client::Context;
use serenity::model::permissions::MANAGE_MESSAGES;
use time;

command!(embed(_ctx, msg, _args, text: String) {
    let s: String = _args.join(" ");
    let jsprs = JsonToDiscordEmbedObject::new();
    let _ = match jsprs.parse(s.as_str()) {
        Ok(r) => msg.channel_id.send_message(|m| m.content(" ").embed(|_| r)),
        Err(e) => msg.channel_id.send_message(|m| m.content(format!("**ERROR**\n```{}```", e).as_str()))
    };
    let _ = msg.delete();
});

command!(purge_self(_ctx, msg, _args, amount: u64) {
    let count: u64 = match amount {
        1...100 => amount,
        _ => 0
    };
    let userid = CACHE.read().unwrap().user.id;
    purge_messages(_ctx, msg, amount, Some(userid));
});

command!(purge(_ctx, msg, _args, amount: u64) {
    let count: u64 = match amount {
        1...100 => amount,
        _ => 0
    };
    purge_messages(_ctx, msg, amount, None);
});
fn purge_messages(_ctx: &mut Context, msg: &Message, amount: u64, user: Option<UserId>) {
     match msg.channel_id.messages(|g| g.limit(100)) {
        Ok(x) => {
            let filtered_result: Vec<MessageId> = x.into_iter().filter(|p| {
                if let Some(uid) = user {
                    if p.author.id != uid {
                        return false;
                    }
                }
                let mut tmstmp = p.timestamp.clone();
                let _ = tmstmp.drain(19..26); // Discard Miliseconds
                let ptime = match time::strptime(tmstmp.as_str(), "%Y-%m-%dT%H:%M:%S%z") {
                    Ok(time) => time,
                    Err(e) => {
                        info!("Timeparse for msg {:?} failed: {:?}", p.id, e);
                        return false;
                    }
                };
                let ctime = time::now();
                let delta = ctime - ptime;
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
        },
        Err(_) => ()
    }
}
