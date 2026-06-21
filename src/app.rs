use eframe::egui;

#[derive(Default)]
pub struct MyApp {
    label: String,
    counter: i32,
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Setup fonts/visuals di sini jika perlu
        // cc.egui_ctx.set_visuals(egui::Visuals::dark());
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("Hello dari egui!");
            ui.horizontal(|ui| {
                ui.label("Label:");
                ui.text_edit_singleline(&mut self.label);
            });
            if ui.button("Klik saya").clicked() {
                self.counter += 1;
            }
            ui.label(format!("Counter: {}", self.counter));
        });
    }
}
