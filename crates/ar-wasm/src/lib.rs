use image::{ImageBuffer, Rgba};
use opencv::core::Vector;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsValue;
use web_sys::console;

const TEMPLATE_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/poi.png"));

#[wasm_bindgen]
pub fn template_match(image_data: Clamped<Vec<u8>>, width: u32, height: u32) -> u8 {
    // Convert the input image data to an image buffer
    let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, image_data.0).unwrap();
    let image = opencv::core::Mat::from_slice(buffer.as_raw()).unwrap();
    let mut image_gray = Mat::default();
    imgproc::cvt_color(&image, &mut image_gray, imgproc::COLOR_RGBA2GRAY, 0).unwrap();

    // Convert the template image from embedded bytes to a Mat object

    // Get the path of the built image file
    let out_dir = env::var("OUT_DIR").unwrap();
    let template_path = PathBuf::from(out_dir).join("aaa.png");

    // Read the template image from the file system
    let template_data = fs::read(template_path).expect("Unable to read template image");
    let template =
        imgcodecs::imdecode(&Vector::from_slice(&template_data), imgcodecs::IMREAD_COLOR).unwrap();

    let mut template_gray = Mat::default();
    imgproc::cvt_color(&template, &mut template_gray, imgproc::COLOR_BGR2GRAY, 0).unwrap();

    // Perform template matching
    let result_cols = image_gray.cols() - template_gray.cols() + 1;
    let result_rows = image_gray.rows() - template_gray.rows() + 1;
    // let mut result =
    //     opencv::core::Mat::zeros(result_rows, result_cols, opencv::core::CV_32FC1).unwrap();
    // let mut result = Mat::zeros(result_rows, result_cols, opencv::core::CV_32FC1)
    //     .expect("Failed to create result matrix");
    let mut result = Mat::default();

    imgproc::match_template(
        &image_gray,
        &template_gray,
        &mut result,
        imgproc::TM_CCOEFF_NORMED,
        &opencv::core::no_array(),
    )
    .unwrap();

    // Check the matching result
    let mut min_val = 0.0;
    let mut max_val = 0.0;
    let mut min_loc = opencv::core::Point::default();
    let mut max_loc = opencv::core::Point::default();

    opencv::core::min_max_loc(
        &result,
        Some(&mut min_val),
        Some(&mut max_val),
        Some(&mut min_loc),
        Some(&mut max_loc),
        &opencv::core::no_array(),
    )
    .unwrap();

    if max_val > 0.8 {
        1
    } else {
        0
    }
}
