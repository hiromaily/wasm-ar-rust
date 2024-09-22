use imageproc::edges::canny;

fn main() {
    // 1. load image
    let img = image::open("./images/mountain.png").expect("Failed to open input image");

    // 2. transform to grayscale
    let gray_img = img.to_luma8();

    // 3. detect Canny Edge / Cannyエッジ検出
    let edges = canny(&gray_img, 50.0, 100.0);

    // 4. save
    edges
        .save("./output/main.png")
        .expect("Failed to save result image");

    println!("Canny edge detection completed and saved as output.png");
}
