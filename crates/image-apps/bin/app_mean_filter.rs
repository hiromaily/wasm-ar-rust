use imageproc::filter::box_filter;

fn main() {
    // 1. load image
    println!("1. load image");
    let img = image::open("./images/mountain.png").expect("Failed to open input image");

    // 2. transform to mean filter / 平均フィルタリング
    println!("2. transform to mean filter");
    let kernel_size = 3;
    let filtered_img = box_filter(&img.to_luma8(), kernel_size, kernel_size);

    // 3. save
    println!("3. save");
    filtered_img
        .save("./output/mean-filter.png")
        .expect("Failed to save result image");
}
