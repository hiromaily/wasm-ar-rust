use imageproc::contrast::{otsu_level, threshold, ThresholdType};

fn main() {
    // 1. load image
    println!("1. load image");
    let img = image::open("./images/mountain.png").expect("Failed to open input image");

    // 2. transform to grayscale
    println!("2. transform to grayscale");
    let gray_img = img.to_luma8();

    // 3. transform to Otsu Thresholding / 二値化
    println!("3. transform to otsu thresholding");
    let level = otsu_level(&gray_img);
    let binary_img = threshold(&(gray_img), level as u8, ThresholdType::Binary);

    // 4. save
    println!("4. save");
    binary_img
        .save("./output/thresholding.png")
        .expect("Failed to save result image");
}
