use serde::Deserialize;
use std::collections::HashMap;
use sysinfo::{SystemExt, ProcessExt};
use colored::Colorize;

#[derive(Deserialize)]
struct Recovery {
	cmd: String,
	cwd: Option<String>,
}

#[derive(Deserialize)]
struct Item {
	name: Option<String>,
	exe: Option<String>,
	recovery: Option<Recovery>,
}

///
/// name "Google Chrome Helper"
/// exe "/Applications/Google Chrome.app/Contents/Versions/74.0.3729.169/Google Chrome Helper.app/Contents/MacOS/Google Chrome Helper"
fn main() {
	if let Ok(config) = std::fs::read_to_string("config.toml") {
		let items: HashMap<String, Item> = toml::from_str(&config).expect(
			r#"Cannot parse config.toml.

--- Example ---
[Finder]
name="Finder"

[Alacritty]
name="alacritty"
recovery = {cmd="alacritty"}

[Docker]
exe='dockerd'
recovery = {cmd="sudo service docker start"}

[server1]
exe='python -m http_server'
recovery = {cmd="python -m http_server", cwd="/var/www"}

[Chrome]
name="Google Chrome"
exe="/Applications/Google Chrome.app"
---------------
"#,
		);
		let sys = sysinfo::System::new();
		let procs = sys.get_process_list();

		for (name, item) in &items {
			print!("{:>10} | ", name.white());

			if item.name.is_none() && item.exe.is_none() {
				println!("{}", "Requires name or exe".red().bold());
				continue;
			}

			let mut found = false;
			for (_pid, proc) in procs {
				if (item.name.is_none() || proc.name().contains(item.name.as_ref().unwrap()))
					&& (item.exe.is_none()
						|| proc
							.exe()
							.to_str()
							.unwrap()
							.contains(item.exe.as_ref().unwrap()))
				{
					found = true;
					break;
				}
			}

			if found {
				println!("{}", "Running".green().bold());
			} else {
				if let Some(recovery) = item.recovery.as_ref() {
					let mut cmd = std::process::Command::new(&recovery.cmd);
					if let Some(cwd) = recovery.cwd.as_ref() {
						cmd.current_dir(cwd);
					}
					if cmd.spawn().is_ok() {
						println!("{}", "Restarting ..".white().bold());
					} else {
						println!("{}", "Failed to restart".red().bold());
					}
				} else {
					println!("{}", "Not Running".red().bold());
				}
			}
		}
	} else {
		println!("Cannot read 'config.toml'.");
	}
}
