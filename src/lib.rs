// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-gpu-performance-monitoring: gpu performance monitoring component model.
//!
//! Real implementation: gpus, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gpu {
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
pub enum GpuPerformanceMonitoringError {
    EmptyGpuName,
    DuplicateGpu(String),
    EmptyEventSubject,
}

impl fmt::Display for GpuPerformanceMonitoringError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGpuName => write!(f, "gpu name must not be empty"),
            Self::DuplicateGpu(id) => write!(f, "duplicate gpu {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for GpuPerformanceMonitoringError {}

pub struct GpuPerformanceMonitoring {
    pub name: String,
    gpus: BTreeMap<String, Gpu>,
    events: BTreeMap<String, Event>,
}

impl GpuPerformanceMonitoring {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            gpus: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_gpu(&mut self, item: Gpu) -> Result<(), GpuPerformanceMonitoringError> {
        if item.name.trim().is_empty() {
            return Err(GpuPerformanceMonitoringError::EmptyGpuName);
        }
        let id = item.id.clone();
        if self.gpus.contains_key(&id) {
            return Err(GpuPerformanceMonitoringError::DuplicateGpu(id.clone()));
        }
        self.gpus.insert(id, item);
        Ok(())
    }

    pub fn gpu_count(&self) -> usize {
        self.gpus.len()
    }

    pub fn get_gpu(&self, id: &str) -> Option<&Gpu> {
        self.gpus.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), GpuPerformanceMonitoringError> {
        if event.subject.trim().is_empty() {
            return Err(GpuPerformanceMonitoringError::EmptyEventSubject);
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

    fn sample_gpu(id: &str, name: &str) -> Gpu {
        Gpu {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = GpuPerformanceMonitoring::new("GpuPerformanceMonitoring");
        assert_eq!(m.name, "GpuPerformanceMonitoring");
        assert_eq!(m.gpu_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_gpu() {
        let mut m = GpuPerformanceMonitoring::new("GpuPerformanceMonitoring");
        m.add_gpu(sample_gpu("a1", "Alpha")).unwrap();
        assert_eq!(m.gpu_count(), 1);
        assert!(m.get_gpu("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = GpuPerformanceMonitoring::new("GpuPerformanceMonitoring");
        assert!(m.add_gpu(sample_gpu("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = GpuPerformanceMonitoring::new("GpuPerformanceMonitoring");
        m.add_gpu(sample_gpu("x", "One")).unwrap();
        assert!(m.add_gpu(sample_gpu("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = GpuPerformanceMonitoring::new("GpuPerformanceMonitoring");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
