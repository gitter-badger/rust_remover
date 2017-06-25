use serenity::client::CACHE;
use time::{Tm, Duration};
use time;
use utils::sharekvp::{StartupTime, ReducedReadyPayload};

// Add additional content
command!(status(_context, message) {
    let ca = CACHE.read().unwrap();
    let uri = ca.user.avatar_url().unwrap();

    let data = _context.data.lock().unwrap();
    let uptime = data.get::<StartupTime>().unwrap();
    let rpayload = data.get::<ReducedReadyPayload>().unwrap();
    let tnow: Tm = time::now();

    if let Err(why) = message.channel_id.send_message(
        |m| m.content(" ").embed(
            |e| e.author(
                |a| a.icon_url(uri.as_str()).name(ca.user.name.as_str())
            ).description(
                format!("**Started**: {}\n**Uptime**: {}\n**RRP**: ```{:?}```", uptime.rfc822(), duration_to_ascii(tnow - uptime.clone()), rpayload).as_ref()
            )
             .title("Status")
        )
    ) {
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
    String::from(format!("{} Weeks, {} Days, {} Hours, {} Minutes, {} Seconds", weeks, days, hours, minutes, seconds))
}
