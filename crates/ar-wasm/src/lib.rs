use anyhow::Context;
use image::{ImageBuffer, Rgba};
use opencv::core::Vector;
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::*;
use std::env;
use std::fs;
use std::path::PathBuf;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use wasm_bindgen::JsValue;
use web_sys::console;

// workspace
use opencv_lib::cv::{converter, grayscale, loader, template}; // Note: change name from `opencv-lib`` to `opencv_lib``

// include_bytes! embeds assets when compiling
const TEMPLATE_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/poi.png"));

pub fn load_embedded_template() -> anyhow::Result<Mat> {
    // バイト配列を opencv の Mat として扱う
    let buf = Vector::<u8>::from_slice(TEMPLATE_IMAGE);

    let template = imgcodecs::imdecode(&buf, imgcodecs::IMREAD_COLOR)
        .context("Failed to decode embedded template image")?;

    if !template.empty() {
        Ok(template)
    } else {
        Err(anyhow::anyhow!("Embedded template image is empty"))
    }
}

#[wasm_bindgen]
pub fn template_match(image_data: Clamped<Vec<u8>>, width: u32, height: u32) -> u8 {
    // let image = converter::convert_to_mat(image_data, width, height).unwrap();
    // let template = loader::load_embedded_template().unwrap();

    // let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, image_data.0).unwrap();
    // let image = opencv::core::Mat::from_slice(buffer.as_raw()).unwrap();

    // let out_dir = env::var("OUT_DIR").unwrap();
    // let template_path = PathBuf::from(out_dir).join("aaa.png");

    0
}
