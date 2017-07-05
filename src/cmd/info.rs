use serenity::client::CACHE;
use serenity::utils::builder::CreateEmbedField;
use serenity::model::{Message, GuildId, UserId, Guild, Role, OnlineStatus, VerificationLevel};
use serenity::client::Context;

use utils::sharekvp::StartupTime;
use utils;
use chrono::{Local, Duration};
use std::vec::Vec;
use std::collections::HashMap;
use std::ops::Deref;

use statics;

#[cfg(feature="memory-stats")]
use psutil;

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
        for guild in ca.guilds.values() {
            if let Ok(g) = guild.read() {
                guild_count += 1;
                channel_count += g.channels.len() as usize;
                for memberid in g.members.keys() {
                    let member = user_ids.entry(*memberid).or_insert(0);
                    *member += 1;
                }
            }
        }
        for count in user_ids.values() {
            user_count += *count as usize;
        }
        user_unique_count = user_ids.len();
        user_duplicate_count = user_count - user_unique_count - guild_count + 1;
    }
    let guild_and_user_count_time = Local::now();
    
    #[allow(unused_assignments)]
    let mut total_mem: f64 = -1.0;
    #[allow(unused_assignments)]
    let mut resident_mem: f64 = -1.0;
    #[allow(unused_assignments)]
    let mut shared_mem: f64 = -1.0;
    #[allow(unused_assignments)]
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
    // Used if PSUtils is present
    #[cfg(feature = "memory-stats")]
    {
        memusagefield = memusagefield.name("Memory Usage").value(&format!("**Thread Count**: {}\n**Total**: {:.2} MB\n**Resident**: {:.2} MB\n**Shared**: {:.2} MB",
                            num_threads,
                            utils::round_with_precision(total_mem, 2),
                            utils::round_with_precision(resident_mem, 2),
                            utils::round_with_precision(shared_mem, 2))
        );
    }

    // Used if PSUtils is not present
    #[cfg(not(feature = "memory-stats"))]
    { 
        memusagefield = memusagefield.name("Memory Usage").value("Memory Usage module unavalible on this platform"); 
    }

    let str_time_init = match time_diff_init.num_nanoseconds() {
        Some(s) => format!("{:.4}ms", utils::nanosecond_to_milisecond(s, 2)),
        None => format!("{}ms", time_diff_init.num_milliseconds())
    };
    let str_time_guilds = match time_diff_guilds.num_nanoseconds() {
        Some(s) => format!("{:.4}ms", utils::nanosecond_to_milisecond(s, 2)),
        None => format!("{}ms", time_diff_guilds.num_milliseconds())
    };
    let str_time_memory = match time_diff_memory.num_nanoseconds() {
        Some(s) => format!("{:.4}ms", utils::nanosecond_to_milisecond(s, 2)),
        None => format!("{}ms", time_diff_memory.num_milliseconds())
    };
    utils::check_message(message.channel_id.send_message(
        |m| m.content(" ").embed(
            |e| e.author(
                |a| a.icon_url(uri.as_str()).name(ca.user.name.as_str())
            )
            .description(&format!("**Started**: {}\n**Uptime**: {}", starttime.to_rfc2822(), duration_to_ascii(tnow.signed_duration_since(starttime.to_owned()))))
            .title("Status")
            .colour(utils::random_color())
            .field(|_| memusagefield)
            .field(|f| f.name("Guild Statistics").value(&format!("*{}* Guilds\n*{}* Channels\n*{}* Users\n | -> *{}* Unique\n | -> *{}* Duplicates", guild_count, channel_count, user_count, user_unique_count, user_duplicate_count)))
            .field(|f| f
                .name("Infos")
                .value(&format!("**Target**: {}\n**Authors**: {}\n**Project Name**: {}\n**Version**: {}", 
                        stcs.target, stcs.cargp_pkg_authors, stcs.cargo_pkg_name, stcs.cargo_pkg_version)))
            .footer(|foot| foot.text(&format!("Processing Time: {} init, {} guildcount, {} memory", str_time_init, str_time_guilds, str_time_memory)))
        )
    ));
    let send_time = Local::now().signed_duration_since(memory_and_procress_time).num_milliseconds();
    debug!("Info Command took {}ms to send.", send_time);
});

#[cfg_attr(feature = "clippy", allow(needless_pass_by_value))]
pub fn guild_info(_: &mut Context, message: &Message, args: Vec<String>) -> Result<(), String> {
    let guildid: GuildId;
    if args.len() >= 1 {
        match args[0].parse::<u64>() {
            Ok(n) => guildid = GuildId::from(n),
            Err(e) => return Err(format!("Unable to parse GuildId: {:?}", e))
        }
    } else {
        guildid = match message.guild_id() {
            Some(gid) => gid,
            None => return Err("Not an Guild".to_owned())
        }
    }

    let tmguild = match guildid.find() {
        Some(g) => g,
        None => return Err("Unable to find Guild".to_owned())
    };
    let readguild = tmguild.read().unwrap();

    let guild: &Guild = readguild.deref();

    let verifiaction_level = match guild.verification_level {
        VerificationLevel::None => "Does not require any verification.",
        VerificationLevel::Low => "Must have a verified email on the user's Discord account.",
        VerificationLevel::Medium => "Must also be a registered user on Discord for longer than 5 minutes.",
        VerificationLevel::High => "Must also be a member of the guild for longer than 10 minutes.",
        VerificationLevel::Higher => "Must have a verified phone on the user's Discord account."
    };

    let guild_owner = match guild.owner_id.get() {
        Ok(u) => format!("{}#{} ({})", u.name, u.discriminator, u.id.0),
        Err(_) => "~ Unable to retrieve Owner".to_owned()
    };


    let channel_count = &guild.channels.len();

    let afk_channel = match guild.afk_channel_id {
        Some(id) => match id.get() {
            Ok(c) => format!("{}", c),
            Err(_) => format!("None (ID: {})", id.0)
        },
        None => "No AFK Channel".to_owned()
    };

    let afk_timeout = guild.afk_timeout;

    let guild_region = &guild.region;

    let users_total = guild.member_count;
    // [Online, Idle, DND]
    let mut users_online = &mut [0, 0, 0];


    for presence in guild.presences.values() {
        if presence.status == OnlineStatus::Online {
            users_online[0] += 1;
        } else if presence.status == OnlineStatus::Idle {
            users_online[1] += 1;
        } else if presence.status == OnlineStatus::DoNotDisturb {
            users_online[2] += 1;
        }
    }

    let users_online_total = users_online[0] + users_online[1] + users_online[2];

    let mut role_string = String::new();
    {
        let mut roles: Vec<Role> = guild.roles.values().cloned().collect();
        utils::quick_sort(&mut roles);
        roles.reverse();
        role_string.push_str(&format!("{:18} {:4} {}\n", "ID", "Pos", "Name"));
        role_string.push_str(&format!("{:18} {:4} {}\n", "---", "---", "---"));
        for role in roles {
            role_string.push_str(&format!("{:18} {:4} {}\n", role.id, role.position, role.name));
        }
    }


    utils::check_message(message.channel_id.send_message(|m| m.content(" ").embed(|e| e
        .title(&format!("Statistics for {}", guild.name))
        .description(&format!("**Owner**: {}\n**Verification**: {}\n**Region**: {}", guild_owner, verifiaction_level, guild_region))
        .field(|f| f
            .name("Channels")
            .value(&format!("**Count**: {}\n**AFK Channel**: {}\n**AFK Timeout**: {}s\n", channel_count, afk_channel, afk_timeout)))
        .field(|f| f
            .name("Users")
            .value(&format!("**Total User**: {}\n**Online Users**: {}\n | -> *Online*: {}\n | -> *Idle*: {}\n | -> *DND*: {}", users_total, users_online_total, users_online[0], users_online[1], users_online[2])))
        .field(|f| f
            .name("Roles")
            .value(&format!("```{}```", role_string))))
    ));

    Ok(())
}


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