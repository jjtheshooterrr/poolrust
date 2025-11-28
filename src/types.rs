use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct AnalysisResult {
    pub free_chlorine_ppm: f32,
    pub ph: f32,
    pub total_alkalinity_ppm: f32,
    pub cyanuric_acid_ppm: f32,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct PadRegion {
    pub name: &'static str,
    // Ratios relative to the strip width/height
    pub x_start_ratio: f32,
    pub x_end_ratio: f32,
    pub y_start_ratio: f32,
    pub y_end_ratio: f32,
}

// Define the layout of the pads on the strip
// This would need to be adjusted to match the specific brand of test strips
pub const PAD_LAYOUT: [PadRegion; 4] = [
    PadRegion {
        name: "free_chlorine",
        x_start_ratio: 0.05,
        x_end_ratio: 0.15,
        y_start_ratio: 0.2,
        y_end_ratio: 0.8,
    },
    PadRegion {
        name: "ph",
        x_start_ratio: 0.25,
        x_end_ratio: 0.35,
        y_start_ratio: 0.2,
        y_end_ratio: 0.8,
    },
    PadRegion {
        name: "total_alkalinity",
        x_start_ratio: 0.45,
        x_end_ratio: 0.55,
        y_start_ratio: 0.2,
        y_end_ratio: 0.8,
    },
    PadRegion {
        name: "cyanuric_acid",
        x_start_ratio: 0.65,
        x_end_ratio: 0.75,
        y_start_ratio: 0.2,
        y_end_ratio: 0.8,
    },
];
