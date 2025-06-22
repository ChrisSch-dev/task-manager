use sysinfo::{System, Pid};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: Pid,
    pub name: String,
    pub cpu: f32,
    pub memory_mb: f32,
}

#[derive(Clone)]
pub struct DiskIoSnapshot {
    pub used_gb: f64,
}

#[derive(Clone)]
pub struct DiskHistory {
    pub disk: String,
    pub history: Vec<DiskIoSnapshot>,
}

impl DiskHistory {
    pub fn new(disk: String) -> Self {
        Self {
            disk,
            history: vec![],
        }
    }
    pub fn push(&mut self, snap: DiskIoSnapshot, max: usize) {
        self.history.push(snap);
        if self.history.len() > max {
            self.history.remove(0);
        }
    }
}

#[derive(Clone)]
pub struct NetIoSnapshot {
    pub rx_mb: f64,
    pub tx_mb: f64,
}

#[derive(Clone)]
pub struct NetHistory {
    pub iface: String,
    pub history: Vec<NetIoSnapshot>,
    pub last_rx: u64,
    pub last_tx: u64,
}

impl NetHistory {
    pub fn new(iface: String) -> Self {
        Self {
            iface,
            history: vec![],
            last_rx: 0,
            last_tx: 0,
        }
    }
    pub fn push(&mut self, snap: NetIoSnapshot, max: usize) {
        self.history.push(snap);
        if self.history.len() > max {
            self.history.remove(0);
        }
    }
}

#[derive(Clone)]
pub struct SystemInfo {
    pub sys: System,
    pub cpu_history: Vec<Vec<f32>>,
    pub mem_history: Vec<f32>,
    pub disk_histories: HashMap<String, DiskHistory>,
    pub net_histories: HashMap<String, NetHistory>,
    pub max_history: usize,
}