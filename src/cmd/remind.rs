use serenity::model::Message;
use serenity::client::Context;
use chrono::Duration;
use std::vec::Vec;

fn add_reminder(context: &mut Context, message: &Message, args: Vec<String>) -> Result<(), String> {
    if args.len() < 2 {
        return Err("To few arguments".to_owned());
    }
    Ok(())
}

fn parse_duration(string: String) -> Duration {
    let (duration, unit) = string.split_at(string.len() - 2);
    Duration::days(0)
}