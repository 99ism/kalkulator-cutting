
mod app;
use crate::app::MyApp;

struct Cutting {
    allowance_percentage: f64,
    total_pieces: u64,
    pattern_sets: u64,
    fabric_lays: u64,
}

impl Cutting {
    fn new(
        allowance_percentage: f64,
        total_pieces: u64,
        pattern_sets: u64,
        fabric_lays: u64,
    ) -> Self {
        Self {
            allowance_percentage,
            total_pieces,
            pattern_sets,
            fabric_lays,
        }
    }
}

fn calculate(
    allowance_percentage: f64,
    quantity: u64,
    min_lays: u64,
    max_lays: u64,
) -> Vec<Cutting> {
    let quantity_percentage = ((quantity as f64 * allowance_percentage) / 100.0).ceil() as u64;
    let quantity_ceil = quantity + quantity_percentage;
    let quantity_floor = quantity - quantity_percentage;
    let mut results: Vec<Cutting> = Vec::new();

    for lays in (min_lays..max_lays).rev() {
        for sets in 1.. {
            let total_pieces = sets * lays;
            let cutting = Cutting::new(allowance_percentage, total_pieces, sets, lays);

            if total_pieces > quantity_ceil {
                break;
            }

            if total_pieces >= quantity_floor {
                results.push(cutting);
            }
        }
    }

    results
}

fn main() {
    use crate::app::MyApp;
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Kalkulator Cutting",
        native_options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
    .unwrap();
}
