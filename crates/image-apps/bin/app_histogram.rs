use imageproc::contrast::equalize_histogram;

fn main() {
    // 1. load image
    println!("1. load image");
    let img = image::open("./images/mountain.png").expect("Failed to open input image");

    // 2. transform to grayscale
    println!("2. transform to grayscale");
    let gray_img = img.to_luma8();

    // 3. transform to Histogram Equalization / ヒストグラム均一化
    println!("3. transform to histogram equalization");
    let equalized_img = equalize_histogram(&gray_img);

    // 4. save
    println!("4. save");
    equalized_img
        .save("./output/histogram.png")
        .expect("Failed to save result image");
}
