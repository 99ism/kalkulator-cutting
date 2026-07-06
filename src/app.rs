use eframe::egui;

use crate::cutting_report::{simulate, CuttingReport};

#[derive(Default)]
pub struct CuttingSimulatorApp {
    allowance_percentage: f64,
    quantity: u64,
    min_layers: u64,
    max_layers: u64,
    reports: Vec<CuttingReport>,
    even_only: bool,                // Untuk filter
    selected_quantity: Option<u64>, // Untuk highlight
}

impl CuttingSimulatorApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Setup fonts/visuals di sini jika perlu
        // cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
}

impl eframe::App for CuttingSimulatorApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::Window::new("Cutting Simulator")
                .resizable(false)
                .show(ui, |ui| {
                    ui.set_width(265.0);
                    ui.set_height(400.0);

                    egui::Grid::new("input").striped(true).show(ui, |ui| {
                        ui.label("Allowance Quantity");
                        let allowance_response = ui.add(
                            egui::DragValue::new(&mut self.allowance_percentage)
                                .suffix(" %")
                                .speed(0.5)
                                .range(0.0..=100.0),
                        );
                        ui.end_row();

                        ui.label("Quantity");
                        let quantity_response = ui.add(
                            egui::DragValue::new(&mut self.quantity)
                                .suffix(" pcs")
                                .speed(1),
                        );
                        ui.end_row();

                        ui.label("Minimum Amparan");
                        let min_lays_response = ui.add(
                            egui::DragValue::new(&mut self.min_layers)
                                .suffix(" ampar")
                                .speed(1)
                                .range(1..=200),
                        );
                        ui.end_row();

                        ui.label("Maximum Amparan");
                        let max_lays_response = ui.add(
                            egui::DragValue::new(&mut self.max_layers)
                                .suffix(" ampar")
                                .speed(1)
                                .range(1..=200),
                        );
                        ui.end_row();

                        // CALCULATION HERE
                        // setiap ada perubahan value -> auto `calculate` dan select `quantity`nya
                        if allowance_response.changed()
                            || quantity_response.changed()
                            || min_lays_response.changed()
                            || max_lays_response.changed()
                        {
                            self.reports = simulate(
                                self.allowance_percentage,
                                self.quantity,
                                self.min_layers,
                                self.max_layers,
                            );

                            self.selected_quantity = Some(self.quantity);
                        }
                    });
                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut self.even_only, "Filter Amparan Genap");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("Reset").clicked() {
                                self.allowance_percentage = 3.0;
                                self.quantity = 100;
                                self.min_layers = 2;
                                self.max_layers = 28;
                                self.reports = simulate(3.0, 100, 2, 28);
                                self.even_only = false;
                                self.selected_quantity = Some(100);
                            }
                        });
                    });

                    // kalau checkbox = checked maka filter results berdasarkan result.fabric_lays % 2 == 0
                    let filtered_result: Vec<&CuttingReport> = self
                        .reports
                        .iter()
                        .filter(|result| !self.even_only || result.layers % 2 == 0)
                        .collect();

                    egui_extras::TableBuilder::new(ui)
                        .striped(true)
                        .resizable(false)
                        .cell_layout(egui::Layout::centered_and_justified(
                            egui::Direction::LeftToRight,
                        ))
                        .column(egui_extras::Column::exact(60.0))
                        .column(egui_extras::Column::exact(60.0))
                        .column(egui_extras::Column::exact(60.0))
                        .column(egui_extras::Column::exact(60.0))
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.strong("Highlight");
                            });
                            header.col(|ui| {
                                ui.strong("Quantity");
                            });
                            header.col(|ui| {
                                ui.strong("Gambar");
                            });
                            header.col(|ui| {
                                ui.strong("Ampar");
                            });
                        })
                        .body(|body| {
                            // yang ditampilin cuma yang di variable filtered_result
                            body.rows(18.0, filtered_result.len(), |mut row| {
                                let cutting = filtered_result[row.index()];

                                let is_highlighted =
                                    self.selected_quantity == Some(cutting.total_pieces);
                                if is_highlighted {
                                    row.set_selected(true);
                                }

                                // untuk interaktif highlight per row
                                row.col(|ui| {
                                    let mut checked = is_highlighted;
                                    if ui.checkbox(&mut checked, "").changed() {
                                        self.selected_quantity = if checked {
                                            Some(cutting.total_pieces)
                                        } else {
                                            None
                                        }
                                    }
                                });

                                row.col(|ui| {
                                    ui.label(cutting.total_pieces.to_string());
                                });
                                row.col(|ui| {
                                    ui.label(cutting.sets.to_string());
                                });
                                row.col(|ui| {
                                    ui.label(cutting.layers.to_string());
                                });
                            })
                        });
                });
        });
    }
}
