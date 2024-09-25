use image::{ImageBuffer, Rgba};

fn apply_mosaic(image: &mut ImageBuffer<Rgba<u8>, Vec<u8>>, block_size: u32) {
    let (width, height) = image.dimensions();

    for y in (0..height).step_by(block_size as usize) {
        for x in (0..width).step_by(block_size as usize) {
            let x_end = (x + block_size).min(width);
            let y_end = (y + block_size).min(height);

            // ピクセル色の平均値を計算する
            let mut pixel_sum = [0u32; 4];
            let mut pixel_count = 0;

            for i in x..x_end {
                for j in y..y_end {
                    let pixel = image.get_pixel(i, j).0;
                    for k in 0..4 {
                        pixel_sum[k] += pixel[k] as u32;
                    }
                    pixel_count += 1;
                }
            }

            let avg_pixel = [
                (pixel_sum[0] / pixel_count) as u8,
                (pixel_sum[1] / pixel_count) as u8,
                (pixel_sum[2] / pixel_count) as u8,
                (pixel_sum[3] / pixel_count) as u8,
            ];

            // 平均色でブロックを塗りつぶす
            for i in x..x_end {
                for j in y..y_end {
                    image.put_pixel(i, j, Rgba(avg_pixel));
                }
            }
        }
    }
}

fn main() {
    // 1. load image
    println!("1. load image");
    let img = image::open("./images/mountain.png").expect("Failed to open input image");
    let mut img_buffer = img.to_rgba8();

    // 2. apply mozaic
    println!("3. apply mozaic");
    let mosaic_size = 3;
    apply_mosaic(&mut img_buffer, mosaic_size);

    // 3. save
    println!("3. save");
    img_buffer
        .save("./output/mozaic.png")
        .expect("Failed to save result image");
}
