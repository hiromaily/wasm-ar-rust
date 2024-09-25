use wasm_bindgen::prelude::*;

mod ball;
// mod three;
use crate::ball::start_ball_impl;
// use crate::three::start_three_impl;

#[wasm_bindgen]
// ball animation
pub fn start_ball() -> Result<(), JsValue> {
    start_ball_impl()
}

// #[wasm_bindgen]
// // 3D
// pub fn start_three() -> Result<(), JsValue> {
//     start_three_impl()
// }

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
