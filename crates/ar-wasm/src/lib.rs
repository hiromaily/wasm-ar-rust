use std::env;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::console;

// workspace
use opencv_lib::cv::{grayscale, loader, template, wasm}; // Note: change name from `opencv-lib`` to `opencv_lib``

// include_bytes! embeds assets when compiling
const TEMPLATE_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/poi.png"));

#[wasm_bindgen]
pub struct ResultData {
    pub x: i32,
    pub y: i32,
    pub max: f64,
}

#[wasm_bindgen]
pub fn template_match(rgba: &[u8], width: u32, height: u32) -> anyhow::Result<ResultData, JsValue> {
    // web image
    let web_image = wasm::convert_rgba_to_mat(rgba, width, height).unwrap();
    // template
    let template = loader::load_embedded_template(TEMPLATE_IMAGE).unwrap();

    // 画像のグレースケール化を行う
    let gray_image = grayscale::convert_to_grayscale(&web_image).unwrap();
    let gray_template = grayscale::convert_to_grayscale(&template).unwrap();

    // テンプレートマッチングを実行する
    let result = template::template_matching(&gray_image, &gray_template).unwrap();

    // テンプレートマッチングの結果を取得する
    let (max_loc, max_val) = template::get_matching_result(&result).unwrap();
    //println!("Max Location: {:?}, Max Value: {:?}", max_loc, max_val);
    let log_message = format!("Max Location: {:?}, Max Value: {:?}", max_loc, max_val);
    console::log_1(&log_message.into());

    Ok(ResultData {
        x: max_loc.x,
        y: max_loc.y,
        max: max_val,
    })
    // Err(e) => {
    //     Err(JsValue::from_str(&format!("{:?}", e)))
    // }
}
