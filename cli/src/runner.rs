use std::path::Path;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use notify::{RecursiveMode, Watcher};

pub struct TestOutcome {
    pub passed: bool,
    pub output: String,
}

pub fn run_test(root: &Path, ex_pkg: &str) -> Result<TestOutcome> {
    let out = Command::new("cargo")
        .args(["test", "-p", ex_pkg])
        .current_dir(root)
        .output()?;

    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&out.stdout));
    combined.push_str(&String::from_utf8_lossy(&out.stderr));

    Ok(TestOutcome {
        passed: out.status.success(),
        output: combined,
    })
}

pub fn tail(output: &str, n: usize) -> String {
    let lines: Vec<&str> = output.lines().filter(|l| !l.trim().is_empty()).collect();
    let start = lines.len().saturating_sub(n);
    lines[start..].join("\n")
}

pub enum WatchEvent {
    Rechecked(TestOutcome),
    Stopped,
}

pub fn watch<F>(root: &Path, ex_pkg: &str, watch_dir: &Path, mut on_event: F) -> Result<()>
where
    F: FnMut(WatchEvent),
{
    let stop = Arc::new(AtomicBool::new(false));
    let stop_handler = stop.clone();
    let _ = ctrlc::set_handler(move || {
        stop_handler.store(true, Ordering::SeqCst);
    });

    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(move |res| {
        let _ = tx.send(res);
    })?;
    watcher.watch(watch_dir, RecursiveMode::Recursive)?;

    while !stop.load(Ordering::SeqCst) {
        match rx.recv_timeout(Duration::from_millis(200)) {
            Ok(Ok(event)) if is_edit(&event.kind) => {
                while rx.recv_timeout(Duration::from_millis(150)).is_ok() {}
                if stop.load(Ordering::SeqCst) {
                    break;
                }
                let outcome = run_test(root, ex_pkg)?;
                on_event(WatchEvent::Rechecked(outcome));
            }
            Ok(_) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    on_event(WatchEvent::Stopped);
    Ok(())
}

fn is_edit(kind: &notify::EventKind) -> bool {
    use notify::EventKind;
    matches!(
        kind,
        EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tail_takes_last_nonempty_lines() {
        let out = "a\n\nb\n\n\nc\nd\n";
        assert_eq!(tail(out, 2), "c\nd");
        assert_eq!(tail(out, 10), "a\nb\nc\nd");
    }
}
