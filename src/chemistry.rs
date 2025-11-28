use palette::{Lab, Srgb, IntoColor};
use std::collections::HashMap;

// Simple implementation of CIE76 (Euclidean distance in Lab)
fn cie76(c1: Lab, c2: Lab) -> f32 {
    let dl = c1.l - c2.l;
    let da = c1.a - c2.a;
    let db = c1.b - c2.b;
    (dl * dl + da * da + db * db).sqrt()
}

pub struct ReferenceColor {
    pub value: f32,
    pub lab: Lab,
}

impl ReferenceColor {
    pub fn new(value: f32, r: u8, g: u8, b: u8) -> Self {
        let rgb = Srgb::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        let lab: Lab = rgb.into_color();
        Self { value, lab }
    }
}

pub struct ChemicalDef {
    pub name: &'static str,
    pub references: Vec<ReferenceColor>,
}

pub fn get_chemical_definitions() -> HashMap<&'static str, ChemicalDef> {
    let mut map = HashMap::new();

    // Example data - these need to be calibrated with real strip colors
    map.insert("free_chlorine", ChemicalDef {
        name: "Free Chlorine",
        references: vec![
            ReferenceColor::new(0.0, 255, 255, 200), // Very light yellow
            ReferenceColor::new(1.0, 200, 100, 200), // Light purple
            ReferenceColor::new(3.0, 150, 50, 150),  // Purple
            ReferenceColor::new(5.0, 100, 0, 100),   // Dark purple
        ],
    });

    map.insert("ph", ChemicalDef {
        name: "pH",
        references: vec![
            ReferenceColor::new(6.8, 255, 200, 100), // Yellow
            ReferenceColor::new(7.2, 255, 150, 50),  // Orange
            ReferenceColor::new(7.6, 255, 100, 50),  // Red-Orange
            ReferenceColor::new(8.0, 200, 50, 50),   // Red
        ],
    });

    map.insert("total_alkalinity", ChemicalDef {
        name: "Total Alkalinity",
        references: vec![
            ReferenceColor::new(40.0, 200, 200, 100),
            ReferenceColor::new(80.0, 100, 200, 100),
            ReferenceColor::new(120.0, 50, 150, 100),
            ReferenceColor::new(180.0, 0, 100, 100),
        ],
    });

    map.insert("cyanuric_acid", ChemicalDef {
        name: "Cyanuric Acid",
        references: vec![
            ReferenceColor::new(0.0, 200, 200, 200),
            ReferenceColor::new(30.0, 150, 150, 150),
            ReferenceColor::new(50.0, 100, 100, 100),
            ReferenceColor::new(100.0, 50, 50, 50),
        ],
    });

    map
}

pub fn map_color_to_value(
    r: f32,
    g: f32,
    b: f32,
    chemical: &str,
    defs: &HashMap<&str, ChemicalDef>,
) -> Option<f32> {
    let def = defs.get(chemical)?;
    let rgb = Srgb::new(r / 255.0, g / 255.0, b / 255.0);
    let lab: Lab = rgb.into_color();

    let mut best_value = 0.0;
    let mut min_dist = f32::MAX;

    // Simple nearest neighbor
    for reference in &def.references {
        let dist = cie76(lab, reference.lab);
        if dist < min_dist {
            min_dist = dist;
            best_value = reference.value;
        }
    }

    // TODO: Implement interpolation between the two closest colors for better accuracy

    Some(best_value)
}
