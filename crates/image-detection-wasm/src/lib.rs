use image::{ImageBuffer, Rgb, Rgba, RgbaImage};
use imageproc::{drawing::draw_hollow_rect_mut, rect::Rect};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
//use web_sys::console;

// workspace
use template_matching::{find_extremes, match_template, MatchTemplateMethod};

// include_bytes! embeds assets when compiling
const TEMPLATE_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/poi-s.png"));

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub raw_data: Vec<u8>,
    pub min_value: f32,
    pub min_value_location: (u32, u32),
}

#[wasm_bindgen]
pub fn detect_image(input: &[u8], width: u32, height: u32) -> JsValue {
    // debug log
    // let log_message = format!(
    //     "input: {:?}, width: {:?}, height: {:?}",
    //     input, width, height
    // );
    // console::log_1(&log_message.into());

    // 1. load image
    let web_img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, input.to_vec()).expect("Failed to create ImageBuffer");
    let web_dyn_img = image::DynamicImage::ImageRgba8(web_img);

    let template_img = image::load_from_memory_with_format(TEMPLATE_IMAGE, image::ImageFormat::Png)
        .expect("Failed to create ImageBuffer from template");

    // 2. to grayscale
    let gray_web_img = web_dyn_img.to_luma32f();
    let gray_template_img = template_img.to_luma32f();

    // 3. template matching
    let result_img = match_template(
        &gray_web_img,
        &gray_template_img,
        MatchTemplateMethod::SumOfSquaredDifferences,
    );

    // 4. find min & max values
    let result = find_extremes(&result_img);
    // println!("Min value: {:?}", result.min_value);
    // println!("Min position: {:?}", result.min_value_location);

    // 5. draw a rectangle for the match location
    let mut img_rgb = web_dyn_img.into_rgb8();
    let (tw, th) = (template_img.width(), template_img.height());
    draw_hollow_rect_mut(
        &mut img_rgb,
        Rect::at(
            result.min_value_location.0 as i32,
            result.min_value_location.1 as i32,
        )
        .of_size(tw, th),
        Rgb([255, 0, 0]),
    );

    // 6. convert result to rgba for web
    let mut rgba_img: RgbaImage = ImageBuffer::new(width, height);
    for (x, y, pixel) in rgba_img.enumerate_pixels_mut() {
        let edge_value = img_rgb.get_pixel(x, y)[0];
        *pixel = Rgba([edge_value, edge_value, edge_value, 255]);
    }

    // 7. return
    let res = Response {
        raw_data: rgba_img.into_raw(),
        min_value: result.min_value,
        min_value_location: result.min_value_location,
    };
    to_value(&res).unwrap()
}
