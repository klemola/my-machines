use hostname::get_hostname;
use mac_address::get_mac_address;
use systemstat::{Platform, System};

pub fn hostname() -> String {
    get_hostname().unwrap_or_default()
}

pub fn mac_address() -> String {
    match get_mac_address() {
        Ok(Some(address)) => address.to_string(),
        _ => "".to_string(),
    }
}

fn pretty_duration(dur: chrono::Duration) -> String {
    format!(
        "{:0>#2}d {:0>#2}h {:0>#2}min",
        dur.num_days(),
        dur.num_hours() - dur.num_days() * 24,
        dur.num_minutes() - dur.num_hours() * 60
    )
}

pub fn system_summary() -> String {
    let sys = System::new();
    let placeholder = "<unknown>".to_string();
    let uptime = match sys.uptime() {
        Ok(dur) => time::Duration::from_std(dur)
            .map(pretty_duration)
            .unwrap_or_else(|_| placeholder.clone()),
        Err(_) => placeholder.clone(),
    };
    let load_avg = sys
        .load_average()
        .map(|val| format!("{:0.2} {:0.2} {:0.2}", val.one, val.five, val.fifteen))
        .unwrap_or(placeholder);

    format!("Uptime: {}, Load average: {}", uptime, load_avg)
}
