use anyhow::Context;
use opencv::{
    core::{self, Mat, MatTraitConst, Point, Vector},
    imgcodecs, imgproc,
};

// load image from file path
fn load_image(path: &str) -> anyhow::Result<opencv::prelude::Mat> {
    let img = imgcodecs::imread(path, imgcodecs::IMREAD_COLOR)
        .with_context(|| format!("Failed to load image '{}'", path))?;

    if !img.empty() {
        Ok(img)
    } else {
        Err(anyhow::anyhow!("Image '{}' is empty", path))
    }
}

// load embeded template image
fn load_embedded_template() -> anyhow::Result<Mat> {
    // include_bytes! embeds assets when compiling
    let data = include_bytes!("../images/template.png"); // path srom main.rs

    // バイト配列を opencv の Mat として扱う
    let buf = Vector::<u8>::from_slice(data);

    let template = imgcodecs::imdecode(&buf, imgcodecs::IMREAD_COLOR)
        .context("Failed to decode embedded template image")?;

    if !template.empty() {
        Ok(template)
    } else {
        Err(anyhow::anyhow!("Embedded template image is empty"))
    }
}

// transform images to grayscale
fn convert_to_grayscale(image: &Mat) -> anyhow::Result<Mat> {
    let mut gray_image = Mat::default();
    imgproc::cvt_color(image, &mut gray_image, imgproc::COLOR_BGR2GRAY, 0)
        .context("Failed to convert image to grayscale")?;
    Ok(gray_image)
}

// execute template matching
fn template_matching(image: &Mat, template: &Mat) -> anyhow::Result<Mat> {
    let mut result = Mat::default();
    imgproc::match_template(
        image,
        template,
        &mut result,
        imgproc::TM_CCOEFF_NORMED,
        &core::no_array(),
    )
    .context("Failed to perform template matching")?;
    Ok(result)
}

// get matching result
fn get_matching_result(result: &Mat, min_val: f64) -> anyhow::Result<Point> {
    let mut min_val = min_val; // threshold
    let mut max_loc = Point::new(0, 0);
    core::min_max_loc(
        result,
        Some(&mut min_val),
        None,
        None,
        Some(&mut max_loc),
        &core::no_array(),
    )
    .context("Failed to get min and max locations from result")?;
    Ok(max_loc)
}

fn main() -> anyhow::Result<()> {
    // テンプレートマッチングを実行するための画像とテンプレート画像を読み込む
    println!("1. load images");
    let image_path = "./images/entireimage.png";
    //let template_path = "./images/template.png";

    let image = load_image(image_path)?;
    //let template = load_image(template_path)?;
    let template = load_embedded_template()?;

    // 画像のグレースケール化を行う
    println!("2. transform images to grayscale");
    let gray_image = convert_to_grayscale(&image)?;
    let gray_template = convert_to_grayscale(&template)?;

    // テンプレートマッチングを実行する
    println!("3. execute template matching");
    let result = template_matching(&gray_image, &gray_template)?;

    // テンプレートマッチングの結果を取得する
    println!("4. get result");
    let min_val = 0.9; // threshold
    let max_loc = get_matching_result(&result, min_val)?;
    println!("Max Location: {:?}", max_loc);
    // Max Location: Point_ { x: 1142, y: 412 }

    Ok(())
}
