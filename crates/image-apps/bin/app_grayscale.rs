fn main() {
    // 1. load image
    println!("1. load image");
    let img = image::open("./images/mountain.png").expect("Failed to open input image");

    // 2. transform to grayscale
    println!("2. transform to grayscale");
    let gray_img = img.to_luma8();

    // 3. save
    println!("3. save");
    gray_img
        .save("./output/grayscale.png")
        .expect("Failed to save result image");
}
