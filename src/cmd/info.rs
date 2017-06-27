use serenity::client::CACHE;
use utils::sharekvp::StartupTime;
use chrono::{Local, Duration};
use serenity::model::{Guild, ChannelId, UserId};
use psutil;
use statics;
use std::vec::Vec;
use std::collections::HashMap;
use std::ops::Deref;
use utils::misc::random_color;

const BYTES_TO_MEGABYTES: f64 = 1f64 / (1024f64 * 1024f64);
// Add additional content
command!(status(_context, message) {
    let ca = match CACHE.read() {
        Ok(cache) => cache,
        Err(_) => return Err("Failed to lock cache".to_owned()),
    };

    let uri = ca.user.avatar_url().unwrap();


    // Starttime
    let starttime;
    {
        let data = _context.data.lock().unwrap();
        starttime = *data.get::<StartupTime>().unwrap();
    }
    let tnow = Local::now();

    // Guild Info
    let mut guild_count: usize = 0;
    let mut channel_count: usize = 0;
    let mut user_count: usize = 0;
    let mut user_duplicate_count: usize = 0;
    let mut user_uniqe_count: usize = 0;

    {
        let mut user_ids: HashMap<UserId, u32> = HashMap::new();
        for (_, guild) in &ca.guilds {
            if let Ok(g) = guild.read() {
                guild_count += 1;
                channel_count += g.channels.len() as usize;
                for (memberid, _) in &g.members {
                    let member = user_ids.entry(*memberid).or_insert(0);
                    *member += 1;
                }
            }
        }
        for (_, count) in &user_ids {
            user_count += *count as usize;
        }
        user_uniqe_count = user_ids.len();
        user_duplicate_count = user_count - user_uniqe_count;
    }
    
    // Memory Statistics
    let processes = match psutil::process::all() {
        Ok(processes) => processes,
        Err(_) => return Err("Failed to read process list".to_owned()),
    };
    let process = match processes.iter().find(|p| p.pid == psutil::getpid()) {
        Some(process) => process,
        None => return Err("Failed to retrieve information on process".to_owned()),
    };
    let threads = process.num_threads;
    let memory = match process.memory() {
        Ok(memory) => memory,
        Err(_) => return Err("Failed to retrieve process memory usage".to_owned()),
    };

    let total_mem;
    let resident_mem;
    let shared_mem;
    {
        total_mem = memory.size as f64 * BYTES_TO_MEGABYTES;
        resident_mem = memory.resident as f64 * BYTES_TO_MEGABYTES;
        shared_mem = memory.share as f64 * BYTES_TO_MEGABYTES;
    }

    let stcs = statics::Statics::get();

    if let Err(why) = message.channel_id.send_message(
        |m| m.content(" ").embed(
            |e| e.author(
                |a| a.icon_url(uri.as_str()).name(ca.user.name.as_str())
            )
            .description(&format!("**Started**: {}\n**Uptime**: {}", starttime.to_rfc2822(), duration_to_ascii(tnow.signed_duration_since(starttime.to_owned()))))
            .title("Status")
            .colour(random_color())
            .field(|f|
                f.name("Memory Usage")
                    .value(
                        &format!("**Thread Count**: {}\n**Total**: {:.2} MB\n**Resident**: {:.2} MB\n**Shared**: {:.2} MB",
                            threads.to_string(),
                            round_with_precision(total_mem, 2),
                            round_with_precision(resident_mem, 2),
                            round_with_precision(shared_mem, 2))))
            .field(|f| f.name("Guild Statistics").value(&format!("*{}* Guilds\n*{}* Channels\n*{}* Users\n-> *{}* Unique\n-> *{}* Duplicates", guild_count, channel_count, user_count, user_uniqe_count, user_duplicate_count)))
            .field(|f| f
                .name("Infos")
                .value(&format!("**Target**: {}\n**Authors**: {}\n**Project Name**: {}\n**Version**: {}", 
                        stcs.TARGET, stcs.CARGO_PKG_AUTHORS, stcs.CARGO_PKG_NAME, stcs.CARGO_PKG_VERSION)))
        )
    ) { // Actual if block begins here
        warn!("Sending status failed because: {:?}", why);
    }
});

fn duration_to_ascii(d: Duration) -> String {
    let mut delta = d;
    let weeks = delta.num_weeks();
    delta = delta - Duration::weeks(weeks);
    let days = delta.num_days();
    delta = delta - Duration::days(days);
    let hours = delta.num_hours();
    delta = delta - Duration::hours(hours);
    let minutes = delta.num_minutes();
    delta = delta - Duration::minutes(minutes);
    let seconds = delta.num_seconds();
    String::from(format!(
        "{}w {}d {}h {}m {}s",
        weeks,
        days,
        hours,
        minutes,
        seconds
    ))
}

#[inline]
fn round_with_precision(num: f64, precision: i32) -> f64 {
    let power = 10f64.powi(precision);
    (num * power).round() / power
}