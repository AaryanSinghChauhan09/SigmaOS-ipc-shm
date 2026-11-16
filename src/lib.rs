// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-ipc-shm: ipc shm component model.
//!
//! Real implementation: shms, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shm {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcShmError {
    EmptyShmName,
    DuplicateShm(String),
    EmptyEventSubject,
}

impl fmt::Display for IpcShmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyShmName => write!(f, "shm name must not be empty"),
            Self::DuplicateShm(id) => write!(f, "duplicate shm {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for IpcShmError {}

pub struct IpcShm {
    pub name: String,
    shms: BTreeMap<String, Shm>,
    events: BTreeMap<String, Event>,
}

impl IpcShm {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            shms: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_shm(&mut self, item: Shm) -> Result<(), IpcShmError> {
        if item.name.trim().is_empty() {
            return Err(IpcShmError::EmptyShmName);
        }
        let id = item.id.clone();
        if self.shms.contains_key(&id) {
            return Err(IpcShmError::DuplicateShm(id.clone()));
        }
        self.shms.insert(id, item);
        Ok(())
    }

    pub fn shm_count(&self) -> usize {
        self.shms.len()
    }

    pub fn get_shm(&self, id: &str) -> Option<&Shm> {
        self.shms.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), IpcShmError> {
        if event.subject.trim().is_empty() {
            return Err(IpcShmError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_shm(id: &str, name: &str) -> Shm {
        Shm {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = IpcShm::new("IpcShm");
        assert_eq!(m.name, "IpcShm");
        assert_eq!(m.shm_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_shm() {
        let mut m = IpcShm::new("IpcShm");
        m.add_shm(sample_shm("a1", "Alpha")).unwrap();
        assert_eq!(m.shm_count(), 1);
        assert!(m.get_shm("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = IpcShm::new("IpcShm");
        assert!(m.add_shm(sample_shm("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = IpcShm::new("IpcShm");
        m.add_shm(sample_shm("x", "One")).unwrap();
        assert!(m.add_shm(sample_shm("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = IpcShm::new("IpcShm");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
