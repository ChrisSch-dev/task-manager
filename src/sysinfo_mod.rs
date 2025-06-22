use sysinfo::{System, SystemExt, CpuExt, ProcessExt, DiskExt, NetworkExt, Signal};
use crate::types::{
    SystemInfo, ProcessInfo, DiskHistory, NetHistory, DiskIoSnapshot, NetIoSnapshot,
};

impl SystemInfo {
    pub fn new() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let n_cores = sys.cpus().len();
        let disk_histories = sys
            .disks()
            .iter()
            .map(|d| {
                (
                    d.name().to_string_lossy().to_string(),
                    DiskHistory::new(d.name().to_string_lossy().to_string()),
                )
            })
            .collect();

        let net_histories = sys
            .networks()
            .iter()
            .map(|(iface, _)| (iface.clone(), NetHistory::new(iface.clone())))
            .collect();

        Self {
            sys,
            cpu_history: vec![vec![0.0; 60]; n_cores], // 60 data points per core
            mem_history: vec![0.0; 60],
            disk_histories,
            net_histories,
            max_history: 60,
        }
    }

    pub fn refresh(&mut self) {
        self.sys.refresh_all();

        // CPU history
        let usages: Vec<f32> = self.sys.cpus().iter().map(|c| c.cpu_usage()).collect();
        for (i, usage) in usages.iter().enumerate() {
            let hist = &mut self.cpu_history[i];
            hist.push(*usage);
            if hist.len() > self.max_history {
                hist.remove(0);
            }
        }
        // Memory history (GB)
        let mem_gb = self.mem_used_gb();
        self.mem_history.push(mem_gb);
        if self.mem_history.len() > self.max_history {
            self.mem_history.remove(0);
        }

        // Disk usage history
        for disk in self.sys.disks() {
            let name = disk.name().to_string_lossy().to_string();
            if !self.disk_histories.contains_key(&name) {
                self.disk_histories.insert(name.clone(), crate::types::DiskHistory::new(name.clone()));
            }
            if let Some(h) = self.disk_histories.get_mut(&name) {
                let usage = (disk.total_space() - disk.available_space()) as f64 / 1_073_741_824.0; // GB used
                h.push(crate::types::DiskIoSnapshot { used_gb: usage }, self.max_history);
            }
        }

        // Network usage history (support hotplug)
        for (iface, data) in self.sys.networks() {
            if !self.net_histories.contains_key(iface) {
                self.net_histories.insert(iface.clone(), crate::types::NetHistory::new(iface.clone()));
            }
            if let Some(h) = self.net_histories.get_mut(iface) {
                let last_rx = h.last_rx;
                let last_tx = h.last_tx;
                let rx = data.received();
                let tx = data.transmitted();
                let rx_delta = if last_rx == 0 { 0 } else { rx.saturating_sub(last_rx) };
                let tx_delta = if last_tx == 0 { 0 } else { tx.saturating_sub(last_tx) };

                // Store as MB/s
                h.push(crate::types::NetIoSnapshot {
                    rx_mb: rx_delta as f64 / 1_048_576.0,
                    tx_mb: tx_delta as f64 / 1_048_576.0,
                }, self.max_history);

                h.last_rx = rx;
                h.last_tx = tx;
            }
        }
    }

    pub fn cpu_usages(&self) -> Vec<f32> {
        self.sys.cpus().iter().map(|c| c.cpu_usage()).collect()
    }

    pub fn cpu_history(&self) -> &Vec<Vec<f32>> {
        &self.cpu_history
    }

    pub fn mem_total_gb(&self) -> f32 {
        self.sys.total_memory() as f32 / 1_048_576.0
    }

    pub fn mem_used_gb(&self) -> f32 {
        self.sys.used_memory() as f32 / 1_048_576.0
    }

    pub fn mem_history(&self) -> &Vec<f32> {
        &self.mem_history
    }

    pub fn swap_total_gb(&self) -> f32 {
        self.sys.total_swap() as f32 / 1_048_576.0
    }

    pub fn swap_used_gb(&self) -> f32 {
        self.sys.used_swap() as f32 / 1_048_576.0
    }

    pub fn processes(&self) -> Vec<ProcessInfo> {
        let mut procs: Vec<_> = self.sys.processes().values().map(|proc| ProcessInfo {
            pid: proc.pid(),
            name: proc.name().to_owned(),
            cpu: proc.cpu_usage(),
            memory_mb: proc.memory() as f32 / 1024.0,
        }).collect();
        procs.sort_by(|a, b| b.cpu.partial_cmp(&a.cpu).unwrap());
        procs
    }

    pub fn kill_process(&mut self, pid: sysinfo::Pid) {
        if let Some(proc) = self.sys.process(pid) {
            let _ = proc.kill();
        }
    }

    pub fn signal_process(&mut self, pid: sysinfo::Pid, signal: Signal) {
        if let Some(proc) = self.sys.process(pid) {
            let _ = proc.send_signal(signal);
        }
    }

    pub fn disk_histories(&self) -> &std::collections::HashMap<String, DiskHistory> {
        &self.disk_histories
    }

    pub fn net_histories(&self) -> &std::collections::HashMap<String, NetHistory> {
        &self.net_histories
    }
}