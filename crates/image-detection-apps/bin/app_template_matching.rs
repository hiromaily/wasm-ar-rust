use image::GrayImage;
//use image::{imageops::resize, imageops::FilterType, GrayImage};
//use ndarray::Array2;

// template matching without library
// Fucking slow

fn template_matching(image: &GrayImage, template: &GrayImage) -> Option<(u32, u32)> {
    let (img_width, img_height) = image.dimensions();
    let (tmpl_width, tmpl_height) = template.dimensions();

    let mut min_diff = f32::MAX;
    let mut min_x = 0;
    let mut min_y = 0;
    let step = 5;

    // テンプレート幅と高さの重みを計算
    let norm_factor = (tmpl_width * tmpl_height) as f32;

    // 画像全体をテンプレートに対して評価
    println!(
        "loop: img-h:{}, img-w:{}, tmpl-h:{}, tmpl-w{}",
        img_height, img_width, tmpl_height, tmpl_width
    );
    for y in (0..=img_height - tmpl_height).step_by(step) {
        for x in (0..=img_width - tmpl_width).step_by(step) {
            let mut diff = 0.0;
            let mut early_stop = false;
            for ty in 0..tmpl_height {
                for tx in 0..tmpl_width {
                    let px = image.get_pixel(x + tx, y + ty)[0];
                    let tp = template.get_pixel(tx, ty)[0];
                    diff += (px as f32 - tp as f32).abs();

                    // 一定の差を超えたら早期停止
                    if diff > min_diff {
                        early_stop = true;
                        break;
                    }
                }
                if early_stop {
                    break;
                }
            }
            if diff < min_diff {
                min_diff = diff;
                min_x = x;
                min_y = y;
            }
        }
    }
    println!("eval done: min_diff: {:?}", min_diff);
    // 7483362.0
    // 4712960.0
    // 4463047.0

    // 閾値を設けて、マッチが見つかったかどうかを評価
    let threshold = 100.0; // 適切な閾値を設定する必要があります
    if min_diff < (threshold * norm_factor) {
        Some((min_x, min_y))
    } else {
        None
    }
}

fn main() {
    // 1. load images
    println!("1. load image");
    let bg_img = image::open("images/web1.png").unwrap();
    //let bg_img = image::open("images/web2.png").unwrap();
    //let bg_img = image::open("images/web_nodata.png").unwrap();
    let template_img = image::open("images/poi-s.png").unwrap();

    // 2. transform to grayscale
    println!("2. transform to grayscale");
    let gray_bg_img = bg_img.to_luma8();
    let gray_template_img = template_img.to_luma8();

    // 3. scale 50% template image when it is larger then target image
    // println!("3. scale template image if needed");
    // while gray_template_img.width() > gray_bg_img.width()
    //     || gray_template_img.height() > gray_bg_img.height()
    // {
    //     gray_template_img = resize(
    //         &gray_template_img,
    //         gray_template_img.width() / 2,
    //         gray_template_img.height() / 2,
    //         FilterType::Nearest,
    //     );
    // }

    // 3. template matching
    println!("3. template matching");
    if let Some((x, y)) = template_matching(&gray_bg_img, &gray_template_img) {
        println!("Match found at ({}, {})", x, y);
    } else {
        println!("not matched");
    }
}
