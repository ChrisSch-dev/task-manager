mod process_table;
mod sysinfo_mod;

use eframe::{egui, epi};
use crate::types::{SystemInfo, ProcessInfo};
use process_table::process_table_ui;
use crate::ui_utils::{
    cpu_usage_graph_ui, mem_usage_graph_ui, disk_usage_graph_ui, net_usage_graph_ui,
};

pub struct TaskManagerApp {
    pub sysinfo: SystemInfo,
    pub process_search: String,
    pub update_interval: f32,
}

impl Default for TaskManagerApp {
    fn default() -> Self {
        Self {
            sysinfo: SystemInfo::new(),
            process_search: String::new(),
            update_interval: 1.0,
        }
    }
}

impl epi::App for TaskManagerApp {
    fn name(&self) -> &str {
        "Rust Task Manager"
    }

    fn update(&mut self, ctx: &egui::Context, _: &mut epi::Frame) {
        self.sysinfo.refresh();

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("System Resource Usage");

            ui.horizontal(|ui| {
                ui.label("Update interval (seconds):");
                ui.add(egui::Slider::new(&mut self.update_interval, 0.5..=5.0).step_by(0.5));
            });

            cpu_usage_graph_ui(ui, self.sysinfo.cpu_history());

            mem_usage_graph_ui(ui, self.sysinfo.mem_history());

            disk_usage_graph_ui(ui, self.sysinfo.disk_histories());

            net_usage_graph_ui(ui, self.sysinfo.net_histories());

            ui.separator();

            // Memory Section (current)
            ui.group(|ui| {
                ui.label(format!(
                    "Memory: {:.2} / {:.2} GB",
                    self.sysinfo.mem_used_gb(),
                    self.sysinfo.mem_total_gb()
                ));
                ui.label(format!(
                    "Swap: {:.2} / {:.2} GB",
                    self.sysinfo.swap_used_gb(),
                    self.sysinfo.swap_total_gb()
                ));
            });

            ui.separator();

            // Process filter/search
            ui.horizontal(|ui| {
                ui.label("Search processes:");
                ui.text_edit_singleline(&mut self.process_search);
            });

            let filtered_procs: Vec<_> = self.sysinfo
                .processes()
                .into_iter()
                .filter(|p| p.name.to_lowercase().contains(&self.process_search.to_lowercase()))
                .collect();

            process_table_ui(ui, &mut self.sysinfo, filtered_procs);
        });

        ctx.request_repaint_after(std::time::Duration::from_secs_f32(self.update_interval));
    }
}