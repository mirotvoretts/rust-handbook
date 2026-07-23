use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Pass,
    Fail,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExerciseState {
    pub status: Status,
    pub src_hash: String,
    pub checked_at: u64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Progress {
    #[serde(default)]
    pub current: Option<String>,
    #[serde(default)]
    pub exercises: BTreeMap<String, ExerciseState>,
}

impl Progress {
    pub fn load(path: &Path) -> Progress {
        match fs::read_to_string(path) {
            Ok(text) => serde_json::from_str(&text).unwrap_or_default(),
            Err(_) => Progress::default(),
        }
    }

    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let text = serde_json::to_string_pretty(self)?;
        fs::write(path, text)?;
        Ok(())
    }

    pub fn status_of(&self, ex_pkg: &str) -> Status {
        self.exercises
            .get(ex_pkg)
            .map(|s| s.status)
            .unwrap_or(Status::Unknown)
    }

    pub fn is_passed(&self, ex_pkg: &str) -> bool {
        self.status_of(ex_pkg) == Status::Pass
    }

    pub fn record(&mut self, ex_pkg: &str, status: Status, src_hash: String) {
        self.exercises.insert(
            ex_pkg.to_string(),
            ExerciseState {
                status,
                src_hash,
                checked_at: now_unix(),
            },
        );
    }

    pub fn cached_status(&self, ex_pkg: &str, current_hash: &str) -> Option<Status> {
        let state = self.exercises.get(ex_pkg)?;
        if state.status != Status::Unknown && state.src_hash == current_hash {
            Some(state.status)
        } else {
            None
        }
    }

    pub fn set_current(&mut self, ex_pkg: &str) {
        self.current = Some(ex_pkg.to_string());
    }
}

pub fn hash_src(src: &Path) -> String {
    match fs::read(src) {
        Ok(bytes) => {
            let mut hasher = Sha256::new();
            hasher.update(&bytes);
            hex(&hasher.finalize())
        }
        Err(_) => String::new(),
    }
}

fn hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_progress_defaults() {
        let p = Progress::default();
        assert_eq!(p.status_of("ex-06-01-x"), Status::Unknown);
        assert!(!p.is_passed("ex-06-01-x"));
        assert!(p.current.is_none());
    }

    #[test]
    fn record_and_query() {
        let mut p = Progress::default();
        p.record("ex-06-01-x", Status::Pass, "abc".into());
        assert!(p.is_passed("ex-06-01-x"));
        assert_eq!(p.status_of("ex-06-01-x"), Status::Pass);
    }

    #[test]
    fn cache_hit_only_on_matching_hash() {
        let mut p = Progress::default();
        p.record("ex-06-01-x", Status::Pass, "abc".into());
        assert_eq!(p.cached_status("ex-06-01-x", "abc"), Some(Status::Pass));
        assert_eq!(p.cached_status("ex-06-01-x", "def"), None);
        assert_eq!(p.cached_status("ex-99-99-y", "abc"), None);
    }

    #[test]
    fn cache_miss_on_unknown_status() {
        let mut p = Progress::default();
        p.record("ex-06-01-x", Status::Unknown, "abc".into());
        assert_eq!(p.cached_status("ex-06-01-x", "abc"), None);
    }

    #[test]
    fn roundtrip_serialization() {
        let mut p = Progress::default();
        p.set_current("ex-06-01-x");
        p.record("ex-06-01-x", Status::Fail, "h".into());
        let text = serde_json::to_string(&p).unwrap();
        let back: Progress = serde_json::from_str(&text).unwrap();
        assert_eq!(back.current.as_deref(), Some("ex-06-01-x"));
        assert_eq!(back.status_of("ex-06-01-x"), Status::Fail);
    }
}
