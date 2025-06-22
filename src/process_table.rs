use eframe::egui::{self, Ui};
use crate::types::{ProcessInfo, SystemInfo};
use sysinfo::{Signal, ProcessExt};

pub fn process_table_ui(ui: &mut Ui, sysinfo: &mut SystemInfo, processes: Vec<ProcessInfo>) {
    ui.heading("Processes (top 30 by CPU):");
    egui::ScrollArea::vertical().max_height(400.0).show(ui, |ui| {
        egui::Grid::new("process_table")
            .striped(true)
            .show(ui, |ui| {
                ui.heading("PID");
                ui.heading("Name");
                ui.heading("CPU %");
                ui.heading("Memory (MB)");
                ui.heading("Actions");
                ui.end_row();

                for proc in processes.iter().take(30) {
                    ui.label(proc.pid.to_string());
                    ui.label(&proc.name);
                    ui.label(format!("{:.2}", proc.cpu));
                    ui.label(format!("{:.2}", proc.memory_mb));
                    ui.horizontal(|ui| {
                        if ui.button("Kill").clicked() {
                            sysinfo.kill_process(proc.pid);
                            sysinfo.refresh(); // Fix: Refresh after action
                        }
                        if ui.button("Suspend").clicked() {
                            sysinfo.signal_process(proc.pid, Signal::Stop);
                            sysinfo.refresh();
                        }
                        if ui.button("Resume").clicked() {
                            sysinfo.signal_process(proc.pid, Signal::Cont);
                            sysinfo.refresh();
                        }
                    });
                    ui.end_row();
                }
            });
    });
}