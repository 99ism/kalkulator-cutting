/// Laporan cutting
/// Kain digelar berlapis (layers), lalu dipotong sesuai pola (sets).
/// Total pieces = layers * sets, dalam rentang +- allowance.
pub struct CuttingReport {
    allowance_percentage: f64,
    pub total_pieces: u64,
    pub sets: u64,
    pub layers: u64,
}

impl CuttingReport {
    fn new(allowance_percentage: f64, total_pieces: u64, sets: u64, layers: u64) -> Self {
        Self {
            allowance_percentage,
            total_pieces,
            sets,
            layers,
        }
    }
}

pub fn simulate(
    allowance_percentage: f64,
    quantity: u64,
    min_layers: u64,
    max_layers: u64,
) -> Vec<CuttingReport> {
    // Hitung batas atas dan bawah dari quantity dengan allowance
    let quantity_percentage = ((quantity as f64 * allowance_percentage) / 100.0).ceil() as u64;
    let quantity_upper = quantity + quantity_percentage;
    let quantity_lower = quantity - quantity_percentage;
    let mut reports: Vec<CuttingReport> = Vec::new();

    // Mulai dari layers terbanyak
    for layers in (min_layers..=max_layers).rev() {
        for sets in 1.. {
            let total_pieces = sets * layers;
            let report = CuttingReport::new(allowance_percentage, total_pieces, sets, layers);

            if total_pieces > quantity_upper {
                break;
            }

            if total_pieces >= quantity_lower {
                reports.push(report);
            }
        }
    }

    reports
}
