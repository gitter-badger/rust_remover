use serenity::model::Message;
use serenity::client::Context;
use chrono::Duration;
use std::vec::Vec;

#[cfg_attr(feature = "clippy", allow(needless_pass_by_value))]
#[allow(unused_variables)] // TODO Fix
pub fn add_reminder(context: &mut Context, message: &Message, args: Vec<String>) -> Result<(), String> {
    if args.len() < 2 {
        return Err("To few arguments".to_owned());
    }
    let ptime = match parse_duration(&args[0]) {
        Ok(d) => d,
        Err(e) => return Err(e)
    };
    let tmp_message = args[1..].join(" ");
    let _ = message.reply(&format!("Duration:\n{:#?}\n\nMessage:\n{}", ptime, tmp_message));
    Ok(())
}

fn parse_duration(string: &str) -> Result<Duration, String> {
    let (duration, unit) = string.split_at(string.len() - 1);
    let time = match duration.parse::<u64>() {
        Ok(v) => v,
        Err(e) => return Err(format!("{:?}", e))
    } as i64;

    match unit {
        "w" => Ok(Duration::weeks(time)),
        "d" => Ok(Duration::days(time)),
        "h" => Ok(Duration::hours(time)),
        "m" => Ok(Duration::minutes(time)),
        _ => Err("Unkown duration unit".to_owned())
    }
}