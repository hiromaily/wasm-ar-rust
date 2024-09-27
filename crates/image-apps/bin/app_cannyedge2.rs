use image::RgbaImage;
use imageproc::edges::canny;

fn main() {
    // 1. load image
    println!("1. load image");
    let img = image::open("./images/mountain.png")
        .expect("Failed to open input image")
        .to_rgba8();

    // 2. specify the rectangle
    let rect_x = 100;
    let rect_y = 100;
    let rect_width = 200;
    let rect_height = 150;

    // 3. Apply Canny edge detection outside of the rectangle
    let edge_image = apply_canny_edge_outside(&img, rect_x, rect_y, rect_width, rect_height);

    // 4. Save the result
    println!("3. save");
    edge_image
        .save("./output/cannyedge2.png")
        .expect("Failed to save result image");
}

fn apply_canny_edge_outside(img: &RgbaImage, x: u32, y: u32, width: u32, height: u32) -> RgbaImage {
    // Convert the image to grayscale
    let gray_img = image::imageops::grayscale(img);

    // Perform Canny edge detection on the entire image
    let edges = canny(&gray_img, 50.0, 100.0);

    // Create a new RGBA image to store the result
    let mut output_img = img.clone();

    // Overlay the edges on the original image, outside of the specified rectangle
    for j in 0..edges.height() {
        for i in 0..edges.width() {
            // Check if the pixel is outside the specified rectangle
            if i < x || i >= x + width || j < y || j >= y + height {
                let edge_pixel = edges.get_pixel(i, j);
                if edge_pixel[0] > 0 {
                    output_img.put_pixel(i, j, image::Rgba([255, 0, 0, 255])); // Red for edges
                }
            }
        }
    }

    output_img
}
