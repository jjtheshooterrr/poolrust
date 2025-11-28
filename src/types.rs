use serde::Serialize;

#[derive(Debug, Clone, Copy)]
pub enum Brand {
    Hth6Way,
    Clorox6Way,
    AquaChek7Way,
}

#[derive(Debug, Clone, Copy)]
pub enum Parameter {
    FreeChlorine,
    TotalChlorine,
    Ph,
    TotalAlkalinity,
    CyanuricAcid,
    Hardness,
    Bromine,
}

#[derive(Debug, Clone, Copy)]
pub struct PadRegion {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub parameter: Parameter,
}

#[derive(Debug, Serialize)]
pub struct AnalysisResult {
    pub free_chlorine_ppm: f32,
    pub ph: f32,
    pub total_alkalinity_ppm: f32,
    pub cyanuric_acid_ppm: f32,
    pub notes: Vec<String>,
}
