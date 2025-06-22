use egui::{Ui, plot::{Plot, Line, Values, Value, Legend}};
use crate::types::{DiskHistory, NetHistory};

pub fn cpu_usage_graph_ui(ui: &mut Ui, history: &Vec<Vec<f32>>) {
    ui.heading("Per-core CPU Usage (History)");
    let plot = Plot::new("cpu_plot").height(120.0).legend(Legend::default());
    plot.show(ui, |plot_ui| {
        for (core_idx, core_history) in history.iter().enumerate() {
            let points: Vec<Value> = core_history
                .iter()
                .enumerate()
                .map(|(i, &usage)| Value::new(i as f64, usage as f64))
                .collect();
            let line = Line::new(Values::from_values(points)).name(format!("CPU {}", core_idx));
            plot_ui.line(line);
        }
    });
}

pub fn mem_usage_graph_ui(ui: &mut Ui, history: &Vec<f32>) {
    ui.heading("Memory Usage History (GB)");
    let points: Vec<Value> = history
        .iter()
        .enumerate()
        .map(|(i, &mem)| Value::new(i as f64, mem as f64))
        .collect();
    let plot = Plot::new("mem_plot").height(100.0);
    plot.show(ui, |plot_ui| {
        let line = Line::new(Values::from_values(points)).name("Memory");
        plot_ui.line(line);
    });
}

pub fn disk_usage_graph_ui(
    ui: &mut Ui,
    histories: &std::collections::HashMap<String, DiskHistory>,
) {
    ui.heading("Disk Usage History (GB used)");
    let plot = Plot::new("disk_plot").height(100.0).legend(Legend::default());
    plot.show(ui, |plot_ui| {
        for (name, hist) in histories {
            if hist.history.is_empty() { continue; } // Fix: skip empty
            let points: Vec<Value> = hist
                .history
                .iter()
                .enumerate()
                .map(|(i, snap)| Value::new(i as f64, snap.used_gb))
                .collect();
            let line = Line::new(Values::from_values(points)).name(name);
            plot_ui.line(line);
        }
    });
}

pub fn net_usage_graph_ui(
    ui: &mut Ui,
    histories: &std::collections::HashMap<String, NetHistory>,
) {
    ui.heading("Network Usage (MB/s, Rx/Tx)");
    let plot = Plot::new("net_plot").height(100.0).legend(Legend::default());
    plot.show(ui, |plot_ui| {
        for (iface, hist) in histories {
            if hist.history.is_empty() { continue; } // Fix: skip empty
            let points_rx: Vec<Value> = hist
                .history
                .iter()
                .enumerate()
                .map(|(i, snap)| Value::new(i as f64, snap.rx_mb))
                .collect();
            let points_tx: Vec<Value> = hist
                .history
                .iter()
                .enumerate()
                .map(|(i, snap)| Value::new(i as f64, snap.tx_mb))
                .collect();
            let line_rx =
                Line::new(Values::from_values(points_rx)).name(format!("{} Rx", iface));
            let line_tx =
                Line::new(Values::from_values(points_tx)).name(format!("{} Tx", iface));
            plot_ui.line(line_rx);
            plot_ui.line(line_tx);
        }
    });
}