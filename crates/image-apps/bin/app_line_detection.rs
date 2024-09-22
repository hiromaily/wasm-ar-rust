use image::{Rgba, RgbaImage};
use imageproc::edges::canny;
use imageproc::hough::{detect_lines, draw_polar_lines, LineDetectionOptions, PolarLine};

// FIXME: something wrong
fn main() {
    // 1. load image
    println!("1. load image");
    let img = image::open("./images/mountain.png").expect("Failed to open input image");

    // 2. transform to grayscale
    println!("2. transform to grayscale");
    let gray_img = img.to_luma8();

    // 3. detect Canny Edge / Cannyエッジ検出
    println!("3. detect canny edge");
    let edges = canny(&gray_img, 50.0, 100.0);

    // 4. detect line / Hough変換による直線検出
    println!("4. detect line by Hough transformation");
    let options = LineDetectionOptions {
        vote_threshold: 100, // tweak
        suppression_radius: 1,
    };
    let lines: Vec<PolarLine> = detect_lines(&edges, options);

    // 5. drwa line to image
    println!("5. draw line to image");
    let mut line_img = RgbaImage::from_pixel(img.width(), img.height(), Rgba([255, 255, 255, 0]));
    draw_polar_lines(&mut line_img, &lines, Rgba([255, 0, 0, 255]));

    // 6. save
    println!("6. save");
    line_img
        .save("./output/line-detection.png")
        .expect("Failed to save result image");
}
