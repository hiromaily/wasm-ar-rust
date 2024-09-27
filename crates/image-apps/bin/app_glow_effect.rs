use image::{Rgba, RgbaImage};
use imageproc::filter::gaussian_blur_f32;

fn main() {
    // 1. load image
    println!("1. load image");
    let img = image::open("./images/mountain.png").expect("Failed to open input image");
    let img = img.to_rgba8();

    // 2. specify the rectangle (640x427)
    let rect_x = 200;
    let rect_y = 100;
    let rect_width = 240;
    let rect_height = 227;
    let glow_width = 30; // glow effect size

    // 3. Create the glow effect around the rectangle
    let glow_img = create_glow_around(
        &img,
        rect_x,
        rect_y,
        rect_width,
        rect_height,
        glow_width,
        10.0,
    );

    // 3. save
    println!("3. save");
    glow_img
        .save("./output/glow-effect.png")
        .expect("Failed to save result image");
}

fn create_glow_around(
    img: &RgbaImage,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    glow_width: u32,
    blur_sigma: f32,
) -> RgbaImage {
    // Create a mask with glow around the specified rectangle
    let mut mask = RgbaImage::new(img.width(), img.height());
    for j in 0..mask.height() {
        for i in 0..mask.width() {
            // Only allow pixels within the glow width outside the rectangle
            if (i < x || i >= x + width || j < y || j >= y + height)
                && (i >= x.saturating_sub(glow_width)
                    && i <= x + width + glow_width
                    && j >= y.saturating_sub(glow_width)
                    && j <= y + height + glow_width)
            {
                *mask.get_pixel_mut(i, j) = Rgba([255, 255, 255, 255]); // around rectangle is white
            } else {
                *mask.get_pixel_mut(i, j) = Rgba([0, 0, 0, 0]); // inside rectangle and outside glow width is transparent
            }
        }
    }

    // Apply Gaussian blur to the mask to create the glow effect
    let blurred_mask = gaussian_blur_f32(&mask, blur_sigma);

    // Apply the glow effect to the original image
    let mut output_img = img.clone();
    for j in 0..blurred_mask.height() {
        for i in 0..blurred_mask.width() {
            let mask_pixel = blurred_mask.get_pixel(i, j);
            // Only apply the glow effect if the mask pixel is not fully transparent
            if mask_pixel[3] > 0 {
                let base_pixel = output_img.get_pixel(i, j);
                let blended_pixel = blend_pixels(base_pixel, mask_pixel);
                output_img.put_pixel(i, j, blended_pixel);
            }
        }
    }

    output_img
}

fn blend_pixels(base: &Rgba<u8>, overlay: &Rgba<u8>) -> Rgba<u8> {
    let alpha_overlay = overlay[3] as f32 / 255.0;
    let alpha_base = base[3] as f32 / 255.0 * (1.0 - alpha_overlay);

    let r = (overlay[0] as f32 * alpha_overlay + base[0] as f32 * alpha_base)
        / (alpha_overlay + alpha_base);
    let g = (overlay[1] as f32 * alpha_overlay + base[1] as f32 * alpha_base)
        / (alpha_overlay + alpha_base);
    let b = (overlay[2] as f32 * alpha_overlay + base[2] as f32 * alpha_base)
        / (alpha_overlay + alpha_base);
    let a = (overlay[3] as f32 + base[3] as f32 * (1.0 - alpha_overlay)).min(255.0);

    Rgba([r as u8, g as u8, b as u8, a as u8])
}
