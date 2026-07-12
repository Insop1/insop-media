use crate::config::Config;
use std::collections::HashMap;
use std::process::Command;

fn run_control(
    config: &Config,
    subcommand: &str,
    value: &str,
    special_command: &HashMap<String, String>,
) {
    if let Some(player) = get_priority_player(config) {
        if let Some(cmd) = special_command.get(&player) {
            let full_cmd = if value.is_empty() {
                // So that full_cmd is a String
                cmd.clone()
            } else {
                format!("{} {}", cmd, value)
            };
            let args = vec!["-c", &full_cmd];
            let _ = Command::new("sh").args(args).status();
        }

        let mut args = vec!["-p", &player, subcommand];
        if !value.is_empty() {
            args.push(value);
        }
        let _ = Command::new("playerctl").args(&args).status();
    }
}

fn get_priority_player(config: &Config) -> Option<String> {
    let output = Command::new("playerctl").args(["-l"]).output().ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let active_players: Vec<&str> = stdout
        .lines()
        .map(|l| l.split('.').next().unwrap_or(l))
        .collect();

    config
        .players
        .iter()
        .find(|p| active_players.contains(&p.as_str()))
        .cloned()
}

pub fn volume(config: &Config, change: &str) {
    run_control(config, "volume", change, &config.special_commands.volume);
}
pub fn toggle(config: &Config) {
    run_control(
        config,
        "play-pause",
        "",
        &config.special_commands.play_pause,
    );
}
pub fn next(config: &Config) {
    run_control(config, "next", "", &config.special_commands.next);
}
pub fn previous(config: &Config) {
    run_control(config, "previous", "", &config.special_commands.previous);
}
