use image::Rgb;
use imageproc::template_matching::{find_extremes, match_template, MatchTemplateMethod};
use imageproc::{drawing::draw_hollow_rect_mut, rect::Rect};

// Fucking slow

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

    println!("3. template matching");
    let result_img: image::ImageBuffer<image::Luma<f32>, Vec<f32>> = match_template(
        &gray_bg_img,
        &gray_template_img,
        MatchTemplateMethod::SumOfSquaredErrors,
    );
    let result = find_extremes(&result_img);

    println!("Min value: {:?}", result.min_value);
    println!("Min position: {:?}", result.min_value_location);

    // 一致位置の矩形を描画
    let mut img_rgb = bg_img.into_rgb8();
    let (tw, th) = (template_img.width(), template_img.height());
    draw_hollow_rect_mut(
        &mut img_rgb,
        Rect::at(
            result.min_value_location.0 as i32,
            result.min_value_location.1 as i32,
        )
        .of_size(tw, th),
        Rgb([255, 0, 0]),
    );

    // save result
    img_rgb.save("output.png").expect("Failed to save image");
}
