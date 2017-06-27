use serenity::client::CACHE;
use utils::sharekvp::StartupTime;
use chrono::{Local, Duration};
use serenity::model::UserId;
#[cfg(feature="memory-stats")]
use psutil;
use statics;
use std::vec::Vec;
use std::collections::HashMap;
use utils::misc::random_color;
use serenity::utils::builder::CreateEmbedField;

#[allow(dead_code)]
const BYTES_TO_MEGABYTES: f64 = 1f64 / (1024f64 * 1024f64);
// Add additional content
command!(status(_context, message) {
    let tnow = Local::now();
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
    let init_and_cache_lock_time = Local::now();
    // Guild Info
    let mut guild_count: usize = 0;
    let mut channel_count: usize = 0;
    let mut user_count: usize = 0;
    let mut user_duplicate_count: usize;
    let mut user_unique_count: usize;

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
        user_unique_count = user_ids.len();
        user_duplicate_count = user_count - user_unique_count - guild_count + 1;
    }
    let guild_and_user_count_time = Local::now();
    
    let mut total_mem: f64 = -1.0;
    let mut resident_mem: f64 = -1.0;
    let mut shared_mem: f64 = -1.0;
    let mut num_threads: String = "-1".to_owned();
    // Memory Statistics
    #[cfg(feature="memory-stats")]
    {
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

        {
            total_mem = memory.size as f64 * BYTES_TO_MEGABYTES;
            resident_mem = memory.resident as f64 * BYTES_TO_MEGABYTES;
            shared_mem = memory.share as f64 * BYTES_TO_MEGABYTES;
            num_threads = threads.to_string();
        }
    }
    let memory_and_procress_time = Local::now();

    let stcs = statics::Statics::get();

    let time_diff_init = init_and_cache_lock_time.signed_duration_since(tnow);
    let time_diff_guilds = guild_and_user_count_time.signed_duration_since(init_and_cache_lock_time);
    let time_diff_memory = memory_and_procress_time.signed_duration_since(guild_and_user_count_time);

    let mut memusagefield = CreateEmbedField::default();
    // Used if PSUtils is not present
    #[cfg(feature = "memory-stats")]
    {
        memusagefield = memusagefield.name("Memory Usage").value(&format!("**Thread Count**: {}\n**Total**: {:.2} MB\n**Resident**: {:.2} MB\n**Shared**: {:.2} MB",
                            num_threads,
                            round_with_precision(total_mem, 2),
                            round_with_precision(resident_mem, 2),
                            round_with_precision(shared_mem, 2))
        );
    }

    // Used if PSUtils is not present
    #[cfg(not(feature = "memory-stats"))]
    { memusagefield = memusagefield.name("Memory Usage").value("Memory Usage module unavalible on this platform"); }
    if let Err(why) = message.channel_id.send_message(
        |m| m.content(" ").embed(
            |e| e.author(
                |a| a.icon_url(uri.as_str()).name(ca.user.name.as_str())
            )
            .description(&format!("**Started**: {}\n**Uptime**: {}", starttime.to_rfc2822(), duration_to_ascii(tnow.signed_duration_since(starttime.to_owned()))))
            .title("Status")
            .colour(random_color())
            .field(|_| memusagefield)
            .field(|f| f.name("Guild Statistics").value(&format!("*{}* Guilds\n*{}* Channels\n*{}* Users\n-> *{}* Unique\n-> *{}* Duplicates", guild_count, channel_count, user_count, user_unique_count, user_duplicate_count)))
            .field(|f| f
                .name("Infos")
                .value(&format!("**Target**: {}\n**Authors**: {}\n**Project Name**: {}\n**Version**: {}", 
                        stcs.target, stcs.cargp_pkg_authors, stcs.cargo_pkg_name, stcs.cargo_pkg_version)))
            .footer(|foot| foot.text(&format!("Processing Time: {}ms init, {}ms guildcount, {}ms memory", time_diff_init.num_milliseconds(), time_diff_guilds.num_milliseconds(), time_diff_memory.num_milliseconds())))
        )
    ) { // Actual if block begins here
        warn!("Sending status failed because: {:?}", why);
    }
    let send_time = Local::now().signed_duration_since(memory_and_procress_time).num_milliseconds();
    debug!("Info Command took {}ms to send.", send_time);
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