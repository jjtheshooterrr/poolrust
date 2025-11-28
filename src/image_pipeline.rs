use anyhow::{Result, anyhow};
use image::{DynamicImage, GenericImageView, Pixel};
use image::io::Reader as ImageReader;
use opencv::{
    core::{Mat, Point2f, Size, Vector, BorderTypes},
    imgproc,
    prelude::*,
    types::VectorOfPoint,
};
use std::io::Cursor;
use crate::types::PadRegion;

pub fn load_image_from_bytes(bytes: &[u8]) -> Result<DynamicImage> {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    Ok(img)
}

// Convert image::DynamicImage to opencv::Mat
// Note: This is a simplified conversion. In a real app, you'd handle formats more robustly.
fn dynamic_image_to_mat(img: &DynamicImage) -> Result<Mat> {
    let rgb = img.to_rgb8();
    let (w, h) = rgb.dimensions();
    let mat = Mat::from_slice(rgb.as_raw())?;
    let mat = mat.reshape(3, h as i32)?;
    // OpenCV uses BGR by default, so convert RGB to BGR
    let mut bgr = Mat::default();
    imgproc::cvt_color(&mat, &mut bgr, imgproc::COLOR_RGB2BGR, 0)?;
    Ok(bgr)
}

fn mat_to_dynamic_image(mat: &Mat) -> Result<DynamicImage> {
    let mut rgb = Mat::default();
    imgproc::cvt_color(mat, &mut rgb, imgproc::COLOR_BGR2RGB, 0)?;
    let size = rgb.size()?;
    let data = rgb.data_bytes()?;
    let img_buf = image::RgbImage::from_raw(size.width as u32, size.height as u32, data.to_vec())
        .ok_or_else(|| anyhow!("Failed to create image buffer"))?;
    Ok(DynamicImage::ImageRgb8(img_buf))
}

pub fn find_strip_and_warp(img: &DynamicImage) -> Result<DynamicImage> {
    let mat = dynamic_image_to_mat(img)?;
    
    // 1. Grayscale
    let mut gray = Mat::default();
    imgproc::cvt_color(&mat, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;

    // 2. Blur
    let mut blurred = Mat::default();
    imgproc::gaussian_blur(&gray, &mut blurred, Size::new(5, 5), 0.0, 0.0, BorderTypes::BORDER_DEFAULT as i32)?;

    // 3. Canny Edge Detection
    let mut edges = Mat::default();
    imgproc::canny(&blurred, &mut edges, 50.0, 150.0, 3, false)?;

    // 4. Find Contours
    let mut contours = VectorOfPoint::new();
    imgproc::find_contours(&edges, &mut contours, imgproc::RETR_EXTERNAL, imgproc::CHAIN_APPROX_SIMPLE, opencv::core::Point::default())?;

    // 5. Find largest contour (assuming it's the strip)
    let mut max_area = 0.0;
    let mut best_contour_idx = -1;
    for i in 0..contours.len() {
        let contour = contours.get(i)?;
        let area = imgproc::contour_area(&contour, false)?;
        if area > max_area {
            max_area = area;
            best_contour_idx = i as i32;
        }
    }

    if best_contour_idx == -1 {
        return Err(anyhow!("No contour found"));
    }

    // 6. Perspective Transform (Simplified: Just returning the original image for now)
    // In a full implementation, you would:
    // - ApproxPolyDP to get 4 corners
    // - Sort corners
    // - GetPerspectiveTransform
    // - WarpPerspective
    
    // For this prototype, we'll assume the user takes a decent photo and we just return the original
    // or a slightly cropped version. To make it compile and run without complex geometry logic:
    Ok(img.clone())
}

pub fn get_pad_colors(img: &DynamicImage, regions: &[PadRegion]) -> Vec<(String, (f32, f32, f32))> {
    let (w, h) = img.dimensions();
    let mut results = Vec::new();

    for region in regions {
        let x0 = (region.x_start_ratio * w as f32) as u32;
        let x1 = (region.x_end_ratio * w as f32) as u32;
        let y0 = (region.y_start_ratio * h as f32) as u32;
        let y1 = (region.y_end_ratio * h as f32) as u32;

        let mut r_sum = 0u64;
        let mut g_sum = 0u64;
        let mut b_sum = 0u64;
        let mut count = 0u64;

        for y in y0..y1 {
            for x in x0..x1 {
                if x >= w || y >= h { continue; }
                let p = img.get_pixel(x, y).to_rgb();
                r_sum += p[0] as u64;
                g_sum += p[1] as u64;
                b_sum += p[2] as u64;
                count += 1;
            }
        }

        if count > 0 {
            results.push((
                region.name.to_string(),
                (
                    r_sum as f32 / count as f32,
                    g_sum as f32 / count as f32,
                    b_sum as f32 / count as f32,
                )
            ));
        } else {
             results.push((region.name.to_string(), (0.0, 0.0, 0.0)));
        }
    }

    results
}
