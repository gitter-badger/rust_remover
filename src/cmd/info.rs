use serenity::client::CACHE;
use utils::sharekvp::StartupTime;
use chrono::{Local, Duration};
//use serenity::model::{Guild, GuildChannel, UserId};
use psutil;
use std::vec::Vec;
//use std::ops::Deref;


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
    let guild_count: u64 = 0;
    let channel_count: u64 = 0;
    let user_count: u64 = 0;

    
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


    if let Err(why) = message.channel_id.send_message(
        |m| m.content(" ").embed(
            |e| e.author(
                |a| a.icon_url(uri.as_str()).name(ca.user.name.as_str())
            )
            .description(&format!("**Started**: {}\n**Uptime**: {}", starttime.to_rfc2822(), duration_to_ascii(tnow.signed_duration_since(starttime.to_owned()))))
            .title("Status")
            .field(|f|
                f.name("Memory Usage")
                    .value(
                        &format!("**Thread Count**: {}\n**Total**: {:.2} MB\n**Resident**: {:.2} MB\n**Shared**: {:.2} MB",
                            threads.to_string(),
                            round_with_precision(total_mem, 2),
                            round_with_precision(resident_mem, 2),
                            round_with_precision(shared_mem, 2))))
            .field(|f| f.name("Guild Statistics").value(&format!("Serving:*{}* Guilds\n*{}* Channels\n*{}* Unique Users", guild_count, channel_count, user_count)))
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