// Command:
// ls -l /sys/class/tty/*/device/driver
// Response:
// lrwxrwxrwx 1 root root 0 Apr  4 16:21 /sys/class/tty/ttyS3/device/driver -> ../../../bus/platform/drivers/serial8250

use std::{fmt::Display, process::Command};

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct SerialInfo {
    pub permissions: String,
    pub owner: String,
    pub group: String,
    pub path: String,
    pub driver: String,
    pub timestamp: usize,
}

impl SerialInfo {
    fn from_str(data: &str) -> Self {
        let splot = data.split(' ').collect::<Vec<_>>();

        let path = str_to_string(splot.get(9))
            .replace("/sys/class/tty", "/dev")
            .replace("/device/driver", "");

        let driver = str_to_string(splot.get(11))
            .split('/')
            .last()
            .map(|s| s.to_string())
            .unwrap_or_default();

        let output = Command::new("bash")
            .args(["-c", &format!("ls -l {}", path)])
            .output()
            .expect("Failed to find bash or ls");

        let timestamp = Command::new("bash")
            .args(["-c", &format!("stat {} -c%Y", path)])
            .output()
            .expect("Failed to find bash or ls");
        let timestamp = String::from_utf8_lossy(&timestamp.stdout)
            .replace('\n', "")
            .parse::<usize>()
            .unwrap_or_default();

        let splot = String::from_utf8_lossy(&output.stdout);
        let splot = splot.split(' ').collect::<Vec<_>>();

        Self {
            permissions: str_to_string(splot.first())[1..].to_string(),
            owner: str_to_string(splot.get(2)),
            group: str_to_string(splot.get(3)),
            path,
            driver,
            timestamp,
        }
    }
}

impl Display for SerialInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:15}{:10}{:10}{:10} {:20}{:20}",
            self.permissions, self.owner, self.group, self.timestamp, self.path, self.driver,
        ))
    }
}

/// Get a header for SerialInfo's Display implementation
pub fn get_header() -> String {
    format!(
        "{:15}{:10}{:10}{:10} {:20}{:20}",
        "Permissions", "Owner", "Group", "Timestamp", "Path", "Driver"
    )
}

fn str_to_string(s: Option<&&str>) -> String {
    s.map(|s| s.to_string()).unwrap_or_default()
}

/// Return all serial ports
pub fn get_serial_ports() -> Vec<SerialInfo> {
    let output = Command::new("bash")
        .args(["-c", "ls -l /sys/class/tty/*/device/driver"])
        .output()
        .expect("Failed to find bash or ls.");
    let out = String::from_utf8_lossy(output.stdout.as_slice());
    let mut serials = vec![];
    for s in out.lines() {
        serials.push(SerialInfo::from_str(s));
    }
    serials
}
