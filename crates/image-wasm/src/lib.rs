use image::{ImageBuffer, Luma};
use imageproc::edges::canny;
use wasm_bindgen::prelude::*;
use web_sys::console;

#[wasm_bindgen]
pub fn process_image(input: &[u8], width: u32, height: u32) -> Vec<u8> {
    // debug log
    let log_message = format!(
        "input: {:?}, width: {:?}, height: {:?}",
        input, width, height
    );
    console::log_1(&log_message.into());

    // 1. load image
    // let img = ImageBuffer::<Luma<u8>, _>::from_raw(width, height, input)
    //     .expect("Failed to create ImageBuffer");
    let img: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, input.to_vec()).expect("Failed to create ImageBuffer");

    // 2. detect canny edge
    let edges = canny(&img, 50.0, 100.0);

    // 3. return
    edges.into_raw()
}
