#[cfg(not(target_arch = "wasm32"))]
mod app;
#[cfg(not(target_arch = "wasm32"))]
mod cutting_report;
#[cfg(not(target_arch = "wasm32"))]
use crate::app::CuttingSimulatorApp;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Kalkulator Cutting",
        native_options,
        Box::new(|cc| Ok(Box::new(CuttingSimulatorApp::new(cc)))),
    )
    .unwrap();
}

// Bin target ini tidak dipakai di web (entry point web ada di WebHandle, src/lib.rs),
// tapi tetap harus ada supaya `trunk build` bisa compile package untuk wasm32.
#[cfg(target_arch = "wasm32")]
fn main() {}
