use image::{ImageBuffer, Luma, Rgba, RgbaImage};
use imageproc::edges::canny;
use wasm_bindgen::prelude::*;
//use web_sys::console;

#[wasm_bindgen]
pub fn process_image(input: &[u8], width: u32, height: u32) -> Vec<u8> {
    // debug log
    // let log_message = format!(
    //     "input: {:?}, width: {:?}, height: {:?}",
    //     input, width, height
    // );
    // console::log_1(&log_message.into());

    // 1. load image
    // let img = ImageBuffer::<Luma<u8>, _>::from_raw(width, height, input)
    //     .expect("Failed to create ImageBuffer");
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, input.to_vec()).expect("Failed to create ImageBuffer");

    // 2. to grayscale
    let gray_img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::from_fn(width, height, |x, y| {
        let pixel = img.get_pixel(x, y);
        let gray =
            (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;
        Luma([gray])
    });

    // 3. detect canny edge
    let edges = canny(&gray_img, 50.0, 100.0);

    // 4. convert result to rgba for web
    let mut rgba_img: RgbaImage = ImageBuffer::new(width, height);
    for (x, y, pixel) in rgba_img.enumerate_pixels_mut() {
        let edge_value = edges.get_pixel(x, y)[0];
        *pixel = Rgba([edge_value, edge_value, edge_value, 255]);
    }

    // 5. return
    rgba_img.into_raw()
}
