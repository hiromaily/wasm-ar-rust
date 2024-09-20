use opencv::{core, imgcodecs, imgproc};

fn main() {
    // テンプレートマッチングを実行するための画像とテンプレート画像を読み込みます。
    println!("1. load image");
    let image = imgcodecs::imread("./images/entireimage.png", imgcodecs::IMREAD_COLOR).unwrap();
    let template = imgcodecs::imread("./images/template.png", imgcodecs::IMREAD_COLOR).unwrap();

    // 画像のグレースケール化を行います。
    println!("2. grayscale transform");
    let mut gray_image = core::Mat::default();
    imgproc::cvt_color(&image, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0).unwrap();
    let mut gray_template = core::Mat::default();
    imgproc::cvt_color(&template, &mut gray_template, imgproc::COLOR_BGR2GRAY, 0).unwrap();

    // テンプレートマッチングを実行します。
    println!("3. execute template matching");
    let mut result = core::Mat::default();
    imgproc::match_template(
        &gray_image,
        &gray_template,
        &mut result,
        imgproc::TM_CCOEFF_NORMED,
        &core::no_array(),
    )
    .unwrap();

    // テンプレートマッチングの結果を取得します。
    println!("4. get result");
    let mut min_val = 0.9; // 閾値
    let mut max_loc = core::Point::new(0, 0);
    core::min_max_loc(
        &result,
        Some(&mut min_val),
        None,
        None,
        Some(&mut max_loc),
        &core::no_array(),
    )
    .unwrap();
    println!("Max Location: {:?}", max_loc);
    // Max Location: Point_ { x: 1142, y: 412 }
}
