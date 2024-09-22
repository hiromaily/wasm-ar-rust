use imageproc::filter::gaussian_blur_f32;

fn main() {
    // 1. load image
    println!("1. load image");
    let img = image::open("./images/mountain.png").expect("Failed to open input image");

    // 2. transform to gaussian blur / ぼかし
    println!("2. transform to gaussian blur");
    let blur_radius = 3.0;
    let blurred = gaussian_blur_f32(&img.to_rgba8(), blur_radius);

    // 3. save
    println!("3. save");
    blurred
        .save("./output/gaussianblur.png")
        .expect("Failed to save result image");
}
