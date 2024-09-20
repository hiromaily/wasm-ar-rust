use anyhow::Context;
use opencv::{
    core::{self, Mat, MatTraitConst, Mat_AUTO_STEP, Point, Size, Vector, CV_8UC3},
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

#[allow(dead_code)]
struct Clamped<T>(T);

// convert `Clamped<Vec<u8>>, width, height` to Mat
fn convert_to_mat(image_data: Clamped<Vec<u8>>, width: u32, height: u32) -> anyhow::Result<Mat> {
    let Clamped(data) = image_data;
    let size = Size::new(width as i32, height as i32);

    // from RGB to Mat
    let mat = Mat::from_slice(&data).with_context(|| "Failed to create Mat from image data")?;
    let mat = unsafe {
        Mat::new_size_with_data_unsafe(
            size,
            CV_8UC3,
            mat.data() as *mut std::ffi::c_void,
            Mat_AUTO_STEP,
        )
    }?;

    Ok(mat)
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
fn get_matching_result(result: &Mat) -> anyhow::Result<(Point, f64)> {
    let mut min_val = 0.9; // threshold
    let mut max_val = 0.0; // important to judge
    let mut max_loc = Point::new(0, 0);
    core::min_max_loc(
        result,
        Some(&mut min_val), // 行列内の最小値を格納する変数への参照 (あまり利用しない)
        Some(&mut max_val), // 行列内の最大値を格納する変数への参照 (この値が高いほど、テンプレートが元画像と高く一致していることを示す)
        None,
        Some(&mut max_loc), // テンプレートが最も一致していると思われる元画像の位置
        &core::no_array(),
    )
    .context("Failed to get min and max locations from result")?;
    Ok((max_loc, max_val))
}

fn main() -> anyhow::Result<()> {
    // テンプレートマッチングを実行するための画像とテンプレート画像を読み込む
    println!("1. load images");
    let image_path = "./images/entireimage.png";
    //let image_path = "./images/entireimage-nodata.png";

    let image = load_image(image_path)?;
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
    let (max_loc, max_val) = get_matching_result(&result)?;
    println!("Max Location: {:?}, Max Value: {:?}", max_loc, max_val);
    //Max Location: Point_ { x: 1142, y: 412 }, Max Value: 0.9999997019767761

    Ok(())
}
