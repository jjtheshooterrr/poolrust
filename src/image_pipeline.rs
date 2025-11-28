use anyhow::Result;
use image::{DynamicImage, GenericImageView, Pixel, ImageReader};
use std::io::Cursor;
use crate::types::{Brand, PadRegion, Parameter, AnalysisResult};

pub fn load_image_from_bytes(bytes: &[u8]) -> Result<DynamicImage> {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    Ok(img)
}

fn pad_regions_for_brand(brand: Brand, img: &DynamicImage) -> Vec<PadRegion> {
    let (w, h) = img.dimensions();

    match brand {
        Brand::Hth6Way => {
            // Rough: 6 pads stacked vertically along strip
            // tweak these after visual testing on your machine
            let pad_height = h / 10;
            let pad_width = w / 3;
            let x = w / 2 - pad_width / 2;

            vec![
                PadRegion { x, y: (h as f32 * 0.20) as u32, width: pad_width, height: pad_height, parameter: Parameter::FreeChlorine },
                PadRegion { x, y: (h as f32 * 0.32) as u32, width: pad_width, height: pad_height, parameter: Parameter::Ph },
                PadRegion { x, y: (h as f32 * 0.44) as u32, width: pad_width, height: pad_height, parameter: Parameter::TotalAlkalinity },
                PadRegion { x, y: (h as f32 * 0.56) as u32, width: pad_width, height: pad_height, parameter: Parameter::CyanuricAcid },
                // Add hardness / TC if needed
            ]
        }
        Brand::Clorox6Way => {
            // Similar vertical layout, but slightly shifted
            let pad_height = h / 10;
            let pad_width = w / 3;
            let x = w / 2 - pad_width / 2;

            vec![
                PadRegion { x, y: (h as f32 * 0.18) as u32, width: pad_width, height: pad_height, parameter: Parameter::FreeChlorine },
                PadRegion { x, y: (h as f32 * 0.30) as u32, width: pad_width, height: pad_height, parameter: Parameter::Ph },
                PadRegion { x, y: (h as f32 * 0.42) as u32, width: pad_width, height: pad_height, parameter: Parameter::TotalAlkalinity },
                PadRegion { x, y: (h as f32 * 0.54) as u32, width: pad_width, height: pad_height, parameter: Parameter::CyanuricAcid },
            ]
        }
        Brand::AquaChek7Way => {
            // AquaChek often has more pads; you can expand these
            let pad_height = h / 10;
            let pad_width = w / 3;
            let x = w / 2 - pad_width / 2;

            vec![
                PadRegion { x, y: (h as f32 * 0.16) as u32, width: pad_width, height: pad_height, parameter: Parameter::FreeChlorine },
                PadRegion { x, y: (h as f32 * 0.26) as u32, width: pad_width, height: pad_height, parameter: Parameter::TotalChlorine },
                PadRegion { x, y: (h as f32 * 0.36) as u32, width: pad_width, height: pad_height, parameter: Parameter::Ph },
                PadRegion { x, y: (h as f32 * 0.46) as u32, width: pad_width, height: pad_height, parameter: Parameter::TotalAlkalinity },
                PadRegion { x, y: (h as f32 * 0.56) as u32, width: pad_width, height: pad_height, parameter: Parameter::CyanuricAcid },
                // hardness / bromine pads below if present
            ]
        }
    }
}

fn sample_region_avg_rgb(img: &DynamicImage, r: &PadRegion) -> [f32; 3] {
    let mut sum_r = 0f32;
    let mut sum_g = 0f32;
    let mut sum_b = 0f32;
    let mut count = 0f32;

    for y in r.y..(r.y + r.height).min(img.height()) {
        for x in r.x..(r.x + r.width).min(img.width()) {
            let p = img.get_pixel(x, y).to_rgb();
            sum_r += p[0] as f32;
            sum_g += p[1] as f32;
            sum_b += p[2] as f32;
            count += 1.0;
        }
    }

    if count == 0.0 {
        return [0.0, 0.0, 0.0];
    }

    [sum_r / count, sum_g / count, sum_b / count]
}

pub fn process_image_for_brand(bytes: &[u8], brand: Brand) -> Result<AnalysisResult> {
    let img = load_image_from_bytes(bytes)?;
    let pads = pad_regions_for_brand(brand, &img);

    let mut fc = None;
    let mut ph = None;
    let mut ta = None;
    let mut cya = None;

    for pad in pads {
        let rgb = sample_region_avg_rgb(&img, &pad);
        match pad.parameter {
            Parameter::FreeChlorine => {
                fc = Some(crate::chemistry::estimate_free_chlorine(brand, rgb));
            }
            Parameter::Ph => {
                ph = Some(crate::chemistry::estimate_ph(brand, rgb));
            }
            Parameter::TotalAlkalinity => {
                ta = Some(crate::chemistry::estimate_total_alkalinity(brand, rgb));
            }
            Parameter::CyanuricAcid => {
                cya = Some(crate::chemistry::estimate_cya(brand, rgb));
            }
            _ => {}
        }
    }

    Ok(AnalysisResult {
        free_chlorine_ppm: fc.unwrap_or(0.0),
        ph: ph.unwrap_or(7.2),
        total_alkalinity_ppm: ta.unwrap_or(80.0),
        cyanuric_acid_ppm: cya.unwrap_or(0.0),
        notes: Vec::new(),
    })
}
