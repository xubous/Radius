use std::fs::OpenOptions;
use std::io::Write;

pub fn save_log_handle_auth(log: String) {
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("log_auth.txt")
    .unwrap();

    writeln!(file, "{}", log).unwrap();
}

pub fn save_log_handle_accounting(log: String) {
    let mut file = OpenOptions::new()
    .create(true)
    .append(true)
    .open("log_monitoring.txt")
    .unwrap();

    writeln!(file, "{}", log).unwrap();
}