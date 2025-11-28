use palette::{Lab, Srgb, IntoColor};
use crate::types::Brand;

struct RefColor {
    lab: Lab,
    value: f32,
}

// Simple implementation of CIE76 (Euclidean distance in Lab)
fn cie76(c1: Lab, c2: Lab) -> f32 {
    let dl = c1.l - c2.l;
    let da = c1.a - c2.a;
    let db = c1.b - c2.b;
    (dl * dl + da * da + db * db).sqrt()
}

// helper: convert sRGB(0–255) → Lab
fn rgb_to_lab(rgb: [f32; 3]) -> Lab {
    let srgb = Srgb::new(rgb[0] / 255.0, rgb[1] / 255.0, rgb[2] / 255.0);
    let lab: Lab = srgb.into_color();
    lab
}

fn closest_value(sample_rgb: [f32; 3], palette: &[RefColor]) -> f32 {
    let sample_lab = rgb_to_lab(sample_rgb);
    let mut best = palette[0].value;
    let mut best_dist = f32::MAX;

    for refc in palette {
        let d = cie76(sample_lab, refc.lab);
        if d < best_dist {
            best_dist = d;
            best = refc.value;
        }
    }

    best
}

// --- Palettes ---

fn hth_fc_palette() -> Vec<RefColor> {
    vec![
        RefColor { lab: rgb_to_lab([240.0, 240.0, 210.0]), value: 0.0 },
        RefColor { lab: rgb_to_lab([230.0, 230.0, 140.0]), value: 1.0 },
        RefColor { lab: rgb_to_lab([220.0, 220.0, 90.0]), value: 3.0 },
        RefColor { lab: rgb_to_lab([210.0, 210.0, 60.0]), value: 5.0 },
        RefColor { lab: rgb_to_lab([200.0, 200.0, 30.0]), value: 10.0 },
    ]
}

fn hth_ph_palette() -> Vec<RefColor> {
    vec![
        RefColor { lab: rgb_to_lab([255.0, 220.0, 170.0]), value: 6.8 },
        RefColor { lab: rgb_to_lab([255.0, 180.0, 160.0]), value: 7.2 },
        RefColor { lab: rgb_to_lab([255.0, 140.0, 150.0]), value: 7.5 },
        RefColor { lab: rgb_to_lab([255.0, 100.0, 140.0]), value: 7.8 },
        RefColor { lab: rgb_to_lab([255.0, 60.0, 130.0]), value: 8.4 },
    ]
}

fn hth_ta_palette() -> Vec<RefColor> {
    vec![
        RefColor { lab: rgb_to_lab([220.0, 240.0, 170.0]), value: 0.0 },
        RefColor { lab: rgb_to_lab([200.0, 230.0, 140.0]), value: 40.0 },
        RefColor { lab: rgb_to_lab([180.0, 220.0, 110.0]), value: 80.0 },
        RefColor { lab: rgb_to_lab([160.0, 210.0, 80.0]), value: 120.0 },
        RefColor { lab: rgb_to_lab([140.0, 200.0, 60.0]), value: 180.0 },
    ]
}

fn hth_cya_palette() -> Vec<RefColor> {
    vec![
        RefColor { lab: rgb_to_lab([255.0, 230.0, 190.0]), value: 0.0 },
        RefColor { lab: rgb_to_lab([255.0, 200.0, 150.0]), value: 30.0 },
        RefColor { lab: rgb_to_lab([255.0, 170.0, 120.0]), value: 50.0 },
        RefColor { lab: rgb_to_lab([255.0, 140.0, 90.0]), value: 100.0 },
    ]
}

// --- Public Estimators ---

pub fn estimate_free_chlorine(brand: Brand, rgb: [f32; 3]) -> f32 {
    let palette = match brand {
        Brand::Hth6Way | Brand::Clorox6Way | Brand::AquaChek7Way => hth_fc_palette(), // Placeholder: use HTH for all for now
    };
    closest_value(rgb, &palette)
}

pub fn estimate_ph(brand: Brand, rgb: [f32; 3]) -> f32 {
    let palette = match brand {
        Brand::Hth6Way | Brand::Clorox6Way | Brand::AquaChek7Way => hth_ph_palette(),
    };
    closest_value(rgb, &palette)
}

pub fn estimate_total_alkalinity(brand: Brand, rgb: [f32; 3]) -> f32 {
    let palette = match brand {
        Brand::Hth6Way | Brand::Clorox6Way | Brand::AquaChek7Way => hth_ta_palette(),
    };
    closest_value(rgb, &palette)
}

pub fn estimate_cya(brand: Brand, rgb: [f32; 3]) -> f32 {
    let palette = match brand {
        Brand::Hth6Way | Brand::Clorox6Way | Brand::AquaChek7Way => hth_cya_palette(),
    };
    closest_value(rgb, &palette)
}
