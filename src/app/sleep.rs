use std::process::Command;
use std::sync::mpsc::Receiver;
use std::thread::{sleep, spawn, JoinHandle};
use std::time::{Duration, Instant};

// Need this as the standard time Instant doesn't implement Default
pub(crate) struct SleepyInstant(pub std::time::Instant);

impl Default for SleepyInstant {
    fn default() -> Self {
        Self(std::time::Instant::now())
    }
}

pub(crate) fn sleep_at(when_to_sleep: Instant, kill_signal: Receiver<bool>) -> JoinHandle<()> {
    spawn(move || {
        let mut now = Instant::now();
        let mut until_sleep: Duration;
        while now < when_to_sleep {
            if kill_signal.try_recv().is_ok() {
                println!("Kill signal received");
                return;
            }
            now = Instant::now();
            until_sleep = when_to_sleep.duration_since(now);
            sleep(Duration::from_secs(1));
            println!("Time until sleep: {:?}", until_sleep);
        }
        println!("Sleeping now");
        let mut cmd = Command::new("osascript");
        cmd.arg("-e").arg("tell application \"Finder\" to sleep");
        println!("Sleep command output: {:?}", cmd.output());
    })
}
