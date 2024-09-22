use image::{ImageBuffer, Rgba, RgbaImage};
use imageproc::edges::canny;
use std::path::Path;

// WIP
fn process_image(input: &[u8], width: u32, height: u32) -> Vec<u8> {
    // 1. load image
    // [Error]: image is not png. RGBA
    // let img = image::load_from_memory_with_format(input, image::ImageFormat::Png)
    //     .expect("Failed to call load_from_memory_with_format()");
    let img: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, input.to_vec()).expect("Failed to create ImageBuffer");

    // 2. to grayscale
    //let gray_img = img.to_luma8();
    let gray_img = image::DynamicImage::ImageRgba8(img).to_luma8();

    // 3. detect canny edge
    let edges = canny(&gray_img, 50.0, 100.0);

    // 4. convert result to rgba for web
    let mut rgba_img: RgbaImage = ImageBuffer::new(width, height);
    for (x, y, pixel) in rgba_img.enumerate_pixels_mut() {
        let edge_value = edges.get_pixel(x, y)[0];
        *pixel = Rgba([edge_value, edge_value, edge_value, 255]);
    }

    // 5. return the raw bytes of the RGBA image
    rgba_img.into_raw()
}

fn main() {
    let input_path = "./images/mountain.png";
    let output_path = "./output/wasm2.png";

    // Read the input image
    let input_img = image::open(Path::new(input_path)).expect("Failed to open input image");
    let input_img = input_img.to_rgba8();
    let (width, height) = input_img.dimensions();
    let input_bytes = input_img.into_raw();

    // call wasm function
    let output_bytes = process_image(&input_bytes, width, height);

    // Convert the raw bytes back to an RGBA image
    let output_img: RgbaImage = ImageBuffer::from_raw(width, height, output_bytes)
        .expect("Failed to create output ImageBuffer");

    // Save the output image
    output_img
        .save(&Path::new(output_path))
        .expect("Failed to save output image");

    println!("Image processing complete. Saved to {}", output_path);
}
