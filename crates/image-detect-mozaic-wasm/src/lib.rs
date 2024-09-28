use anyhow::anyhow;
use image::{DynamicImage, GenericImage, ImageBuffer, Pixel, Rgba, RgbaImage};
use imageproc::{drawing::draw_hollow_rect_mut, edges::canny, rect::Rect};
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use web_sys::console;
mod effect;

use crate::effect::apply_mosaic;

// workspace
use template_matching::{find_extremes, match_template, MatchTemplateMethod};

//
// private
//

// include_bytes! embeds assets when compiling
const TEMPLATE_IMAGE: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/poi-s.png"));

// Helper for error conversion to JsValue
fn convert_error(error: anyhow::Error) -> JsValue {
    JsValue::from_str(&format!("{:?}", error))
}

type LoadImageResult = anyhow::Result<(ImageBuffer<Rgba<u8>, Vec<u8>>, DynamicImage, DynamicImage)>;

fn load_images(input: &[u8], width: u32, height: u32) -> LoadImageResult {
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, input.to_vec())
        .ok_or_else(|| anyhow!("Failed to create ImageBuffer"))?;
    let dyn_img = DynamicImage::ImageRgba8(img.clone());

    let template_img = image::load_from_memory_with_format(TEMPLATE_IMAGE, image::ImageFormat::Png)
        .map_err(|_| anyhow!("Failed to create ImageBuffer from template"))?;

    Ok((img, dyn_img, template_img))
}

async fn template_matching_and_find_extremes(
    img: &DynamicImage,
    template_img: &DynamicImage,
) -> anyhow::Result<(f32, (u32, u32))> {
    // to grayscale
    let gray_img = img.to_luma32f();
    let gray_template_img = template_img.to_luma32f();

    let result_img = match_template(
        &gray_img,
        &gray_template_img,
        MatchTemplateMethod::SumOfSquaredDifferences,
    )
    .await;

    let result = find_extremes(&result_img);
    Ok((result.min_value, result.min_value_location))
}

fn create_response<I>(
    img: &mut I,
    width: u32,
    height: u32,
    min_value: f32,
    min_value_location: (u32, u32),
) -> ImageAndLocationResponse
where
    I: GenericImage,
    I::Pixel: Pixel<Subpixel = u8> + 'static,
{
    // convert result to rgba for web
    let mut rgba_img: RgbaImage = ImageBuffer::new(width, height);
    for (x, y, pixel) in rgba_img.enumerate_pixels_mut() {
        let rgb_pixel = img.get_pixel(x, y).to_rgb();
        *pixel = Rgba([rgb_pixel[0], rgb_pixel[1], rgb_pixel[2], 255]); // to RGBA
    }

    ImageAndLocationResponse {
        raw_data: rgba_img.into_raw(),
        min_value,
        min_value_location,
    }
}

// line color is red when canny edge
#[allow(dead_code)]
fn create_response2<I>(
    img: &mut I,
    width: u32,
    height: u32,
    min_value: f32,
    min_value_location: (u32, u32),
) -> ImageAndLocationResponse
where
    I: GenericImage,
    I::Pixel: Pixel<Subpixel = u8> + 'static,
{
    // convert result to rgba for web
    let mut rgba_img: RgbaImage = ImageBuffer::new(width, height);
    for (x, y, pixel) in rgba_img.enumerate_pixels_mut() {
        let rgb_pixel = img.get_pixel(x, y);
        // Convert the pixel to RGBA
        let channels = rgb_pixel.channels(); // Get channels as a slice
        *pixel = Rgba([
            channels.first().copied().unwrap_or(0),
            channels.get(1).copied().unwrap_or(0),
            channels.get(2).copied().unwrap_or(0),
            255,
        ]);
    }

    ImageAndLocationResponse {
        raw_data: rgba_img.into_raw(),
        min_value,
        min_value_location,
    }
}

//
// public
//

#[derive(Serialize, Deserialize)]
pub struct ImageAndLocationResponse {
    pub raw_data: Vec<u8>,
    pub min_value: f32,
    pub min_value_location: (u32, u32),
}

#[wasm_bindgen]
pub struct ImageDetector {
    effect_mode: u8,                      // effect mode: 1: mozaic, 2: canny edge
    detect_count: u32,                    // template image detected count
    max_detect_count: u32,                // maximum of template image detected count
    matching_threshold: f32,              // threshold for result of template matching
    is_rectangle: bool,                   // enabled drawing rectangle on detected area
    rectangle_color: [u8; 4],             // rectangle color
    prev_valid_min_value_loc: (u32, u32), // previous detected min_value_location as cache
}

impl Default for ImageDetector {
    fn default() -> Self {
        // TODO: add more parameter in frontend part
        Self::new(1, 50, 4000.0, true)
    }
}

#[wasm_bindgen]
impl ImageDetector {
    #[wasm_bindgen(constructor)]
    // TODO: add more parameter in frontend part
    pub fn new(
        effect_mode: u8,
        max_detect_count: u32,
        matching_threshold: f32,
        is_rectangle: bool,
    ) -> ImageDetector {
        ImageDetector {
            effect_mode,
            detect_count: 0,
            max_detect_count,
            matching_threshold,
            is_rectangle,
            rectangle_color: [0, 0, 0, 0],
            prev_valid_min_value_loc: (0, 0),
        }
    }

    pub fn increment(&mut self) -> u32 {
        console::log_1(&format!("increment() max_count: {:?}", self.max_detect_count).into());
        console::log_1(&" increment()".to_string().into());
        self.detect_count += 1;
        if self.detect_count > self.max_detect_count {
            self.detect_count = 1;
        }
        self.detect_count
    }

    pub fn decrement(&mut self) -> u32 {
        console::log_1(&" decrement()".to_string().into());
        if self.detect_count == 0 {
            return 0;
        }
        self.detect_count -= 1;
        self.detect_count
    }

    pub async fn detect_image_and_mozaic(
        &mut self,
        input: &[u8],
        width: u32,
        height: u32,
    ) -> Result<JsValue, JsValue> {
        console_error_panic_hook::set_once();

        let mut is_detected = false;

        // error handling to avoid panic
        let result = async {
            // 1. load image
            //console::log_1(&"1. load image".to_string().into());
            let (mut web_img, web_dyn_img, template_img) = load_images(input, width, height)?;

            // 2. template matching (async function)
            //console::log_1(&"2. template matching".to_string().into());
            let (min_value, min_value_location) =
                template_matching_and_find_extremes(&web_dyn_img, &template_img).await?;

            let count = if min_value > self.matching_threshold {
                // no detection
                self.decrement()
            } else {
                // detected
                is_detected = true;
                // save min_value_location
                self.prev_valid_min_value_loc = min_value_location;

                self.increment()
            };

            // debug
            //console::log_1(&format!("min_value: {:?}, count: {:?}", min_value, count).into());

            if count != 0 {
                // 3. apply effect
                if self.effect_mode == 1 {
                    // mozaic except the match location
                    console::log_1(&"3. apply mozaic".to_string().into());
                    apply_mosaic(
                        &mut web_img,
                        count,
                        if is_detected {
                            min_value_location.0
                        } else {
                            self.prev_valid_min_value_loc.0
                        },
                        if is_detected {
                            min_value_location.1
                        } else {
                            self.prev_valid_min_value_loc.1
                        },
                        template_img.width(),
                        template_img.height(),
                    );
                } else {
                    // canny edge
                    let gray_img = web_dyn_img.to_luma8();
                    let mut edges_img: ImageBuffer<image::Luma<u8>, Vec<u8>> =
                        canny(&gray_img, 50.0, 100.0);

                    let res = create_response(
                        &mut edges_img,
                        width,
                        height,
                        min_value,
                        min_value_location,
                    );
                    return to_value(&res)
                        .map_err(|e| anyhow::anyhow!("Failed to serialize response: {:?}", e));
                }
            }

            // 4. draw a rectangle for the match location
            if self.is_rectangle && is_detected {
                draw_hollow_rect_mut(
                    &mut web_img,
                    Rect::at(min_value_location.0 as i32, min_value_location.1 as i32)
                        .of_size(template_img.width(), template_img.height()),
                    Rgba(self.rectangle_color),
                );
            }

            let res = create_response(&mut web_img, width, height, min_value, min_value_location);
            to_value(&res).map_err(|e| anyhow::anyhow!("Failed to serialize response: {:?}", e))
        }
        .await;

        //console::log_1(&"7. final return".to_string().into());
        //result.map_err(|e| JsValue::from_str(&format!("{:?}", e)))
        result.map_err(convert_error)
    }
}
