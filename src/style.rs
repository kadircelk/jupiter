use std::process::Command;
use std::collections::HashMap;

use term_size;

/// Get the width and height of the terminal window.
pub fn get_term_size() -> (u16, u16) {
    if let Some((w, h)) = term_size::dimensions() {
        return (w as u16, h as u16);
    } else {
        return (80, 24); // Default size if unable to determine terminal size.
    }
}

/// Execute a shell command and return its output as a string.
pub fn command_output(command: &str) -> String {
    let output = Command::new("sh").arg("-c").arg(command).output().unwrap();
    return String::from_utf8(output.stdout).unwrap().trim().to_string();
}

/// Resolve placeholders in a string with actual system information.
pub fn resolve_string(s: &str) -> String {
    let mut resolved_string = String::from(s);

    // Define commands for placeholder resolution.
    // TODO: Make this area more cleaner. Maybe resolving a json file?
    let mut commands: HashMap<&str, &dyn Fn() -> String> = HashMap::new();
    commands.insert("user", &|| command_output("whoami"));
    commands.insert("hostname", &|| command_output("hostname"));
    commands.insert("pwd", &|| command_output("pwd"));
    commands.insert("hour", &|| command_output("date +%H"));
    commands.insert("minute", &|| command_output("date +%M"));
    commands.insert("second", &|| command_output("date +%S"));
    commands.insert("year", &|| command_output("date +%Y"));
    commands.insert("month", &|| command_output("date +%m"));
    commands.insert("day", &|| command_output("date +%d"));
    commands.insert("time", &|| command_output("date +%H:%M:%S"));
    commands.insert("day_of_week", &|| command_output("date +%A"));
    commands.insert("day_of_month", &|| command_output("date +%d"));
    commands.insert("day_of_year", &|| command_output("date +%j"));
    commands.insert("week_of_year", &|| command_output("date +%V"));
    commands.insert("month_of_year", &|| command_output("date +%m"));
    commands.insert("year_of_century", &|| command_output("date +%C"));
    commands.insert("cpu_usage", &|| command_output(r"top -bn1 | grep '^%Cpu' | sed 's/.*, *\([0-9.]*\)%* sy.*/\1/'"));
    commands.insert("memory_usage", &|| command_output("free | grep Mem | awk '{print $3/$2 * 100.0}'"));
    commands.insert("disk_usage", &|| command_output("df -h / | awk '$NF==\"/\"{printf \"%s\", $5}'"));
    commands.insert("uptime", &|| command_output("uptime -p"));
    commands.insert("load_average", &|| command_output("cat /proc/loadavg | awk '{print $1, $2, $3}'"));
    commands.insert("logged_in_users", &|| command_output("who | awk '{print $1}' | sort -u"));
    commands.insert("network_info", &|| command_output("ip addr show"));
    commands.insert("process_count", &|| command_output("ps aux | wc -l"));
    commands.insert("public_ip", &|| command_output("curl ifconfig.me"));
    commands.insert("private_ip", &|| command_output("hostname -I"));
    commands.insert("kernel_version", &|| command_output("uname -r"));
    commands.insert("system_architecture", &|| command_output("uname -m"));

    // Iterate through commands and replace placeholders with actual values.
    for (r, v) in &commands {
        if resolved_string.contains(&format!("{{{}}}", r)) {
            let value = v();
            resolved_string = resolved_string.replace(&format!("{{{}}}", r), &value);
        }
    }

    return resolved_string;
}
