use anyhow::anyhow;
use image::{DynamicImage, ImageBuffer, Rgba, RgbaImage};
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
    max_count: u32,
    threshold: f32,
    call_count: u32,
}

impl Default for ImageDetector {
    fn default() -> Self {
        Self::new(50, 4000.0)
    }
}

#[wasm_bindgen]
impl ImageDetector {
    #[wasm_bindgen(constructor)]
    pub fn new(max_count: u32, threshold: f32) -> ImageDetector {
        ImageDetector {
            max_count,
            threshold,
            call_count: 0,
        }
    }

    // #[wasm_bindgen]
    // pub fn get_call_count(&self) -> u32 {
    //     self.call_count
    // }

    pub fn increment(&mut self) -> u32 {
        console::log_1(&format!("increment() max_count: {:?}", self.max_count).into());
        console::log_1(&" increment()".to_string().into());
        self.call_count += 1;
        if self.call_count > self.max_count {
            self.call_count = 1;
        }
        self.call_count
    }

    pub fn decrement(&mut self) -> u32 {
        console::log_1(&" decrement()".to_string().into());
        //self.call_count = self.call_count.wrapping_sub(1);
        if self.call_count == 0 {
            return 0;
        }
        self.call_count -= 1;
        self.call_count
    }

    pub async fn detect_image_and_mozaic(
        &mut self,
        input: &[u8],
        width: u32,
        height: u32,
    ) -> Result<JsValue, JsValue> {
        console_error_panic_hook::set_once();

        // error handling to avoid panic
        let result = async {
            // 1. load image
            //console::log_1(&"1. load image".to_string().into());
            let (mut web_img, web_dyn_img, template_img) = load_images(input, width, height)?;

            // 2. template matching (async function)
            //console::log_1(&"2. template matching".to_string().into());
            let (min_value, min_value_location) =
                template_matching_and_find_extremes(&web_dyn_img, &template_img).await?;

            let count = if min_value > self.threshold {
                // no detection
                //self.decrement()
                self.decrement()
            } else {
                self.increment()
            };

            // debug
            console::log_1(&format!("min_value: {:?}, count: {:?}", min_value, count).into());

            if count != 0 {
                // 3. apply mozaic except the match location
                console::log_1(&"3. apply mozaic".to_string().into());
                //let mosaic_size = 5;
                apply_mosaic(
                    &mut web_img,
                    count,
                    min_value_location.0,
                    min_value_location.1,
                    template_img.width(),
                    template_img.height(),
                );
            }

            // 4. convert result to rgba for web
            //console::log_1(&"4. convert result to rgba for web".to_string().into());
            let mut rgba_img: RgbaImage = ImageBuffer::new(width, height);
            for (x, y, pixel) in rgba_img.enumerate_pixels_mut() {
                let rgb_pixel = web_img.get_pixel(x, y);
                *pixel = Rgba([rgb_pixel[0], rgb_pixel[1], rgb_pixel[2], 255]); // to RGBA
            }

            // 5. return
            //console::log_1(&"5 return inner function".to_string().into());
            let res = ImageAndLocationResponse {
                raw_data: rgba_img.into_raw(),
                min_value, // under 3000 would be threshold
                min_value_location,
            };

            to_value(&res).map_err(|e| anyhow::anyhow!("Failed to serialize response: {:?}", e))
        }
        .await;

        //console::log_1(&"6. final return".to_string().into());
        //result.map_err(|e| JsValue::from_str(&format!("{:?}", e)))
        result.map_err(convert_error)
    }
}
