use anyhow::Context;
use opencv::{
    core::{
        rotate, Mat, Point, Size, BORDER_DEFAULT, ROTATE_180, ROTATE_90_CLOCKWISE,
        ROTATE_90_COUNTERCLOCKWISE,
    },
    imgproc,
};

// ぼかし (ガウシアンブラー)
// e.g let blurred_image = apply_gaussian_blur(&image, 5)?;
pub fn apply_gaussian_blur(image: &Mat, ksize: i32) -> anyhow::Result<Mat> {
    // カーネルサイズが奇数であることを保証
    let ksize = if ksize % 2 == 0 { ksize + 1 } else { ksize };

    let mut blurred = Mat::default();
    imgproc::gaussian_blur(
        image,
        &mut blurred,
        Size::new(ksize, ksize),
        0.0,
        0.0,
        BORDER_DEFAULT,
    )
    .context("Failed to apply Gaussian blur")?;
    Ok(blurred)
}

// 二値化*
// e.g. let binary_image = apply_threshold(&image, 128.0, 255.0)?;
pub fn apply_threshold(image: &Mat, thresh: f64, max_val: f64) -> anyhow::Result<Mat> {
    let mut binary_image = Mat::default();
    imgproc::threshold(
        image,
        &mut binary_image,
        thresh,
        max_val,
        imgproc::THRESH_BINARY,
    )
    .context("Failed to apply binary threshold")?;
    Ok(binary_image)
}

// エッジ検出 (キャニーエッジ検出)
// e.g. let edges = apply_canny_edge(&image, 100.0, 200.0)?;
pub fn apply_canny_edge(
    image: &Mat,
    low_threshold: f64,
    high_threshold: f64,
) -> anyhow::Result<Mat> {
    let mut edges = Mat::default();
    imgproc::canny(image, &mut edges, low_threshold, high_threshold, 3, false)
        .context("Failed to apply Canny edge detection")?;
    Ok(edges)
}

// リサイズ
// e.g. let resized_image = resize_image(&image, 200, 200)?;
pub fn resize_image(image: &Mat, width: i32, height: i32) -> anyhow::Result<Mat> {
    let mut resized = Mat::default();
    imgproc::resize(
        image,
        &mut resized,
        Size::new(width, height),
        0.0,
        0.0,
        imgproc::INTER_LINEAR,
    )
    .context("Failed to resize image")?;
    Ok(resized)
}

// 回転
// e.g. let rotated_image = rotate_image(&image, 45.0)?;
pub fn rotate_image(image: &Mat, angle: i32) -> anyhow::Result<Mat> {
    let mut rotated = Mat::default();
    let rotate_code = match angle {
        90 => ROTATE_90_CLOCKWISE,
        180 => ROTATE_180,
        270 => ROTATE_90_COUNTERCLOCKWISE,
        _ => {
            return Err(anyhow::anyhow!(
                "Unsupported rotation angle. Only 90, 180, and 270 degrees are supported."
            ))
        }
    };
    rotate(image, &mut rotated, rotate_code).context("Failed to rotate image")?;
    Ok(rotated)
}

// 平滑化 (平均ブラー)
// e.g. let blurred_image = apply_blur(&image, 5)?;
pub fn apply_blur(image: &Mat, ksize: i32) -> anyhow::Result<Mat> {
    let mut blurred = Mat::default();
    imgproc::blur(
        image,
        &mut blurred,
        Size::new(ksize, ksize),
        Point::new(-1, -1),
        BORDER_DEFAULT,
    )
    .context("Failed to apply blur")?;
    Ok(blurred)
}

// HSV変換
// e.g. let hsv_image = convert_to_hsv(&image)?;
pub fn convert_to_hsv(image: &Mat) -> anyhow::Result<Mat> {
    let mut hsv_image = Mat::default();
    imgproc::cvt_color(image, &mut hsv_image, imgproc::COLOR_BGR2HSV, 0)
        .context("Failed to convert image to HSV")?;
    Ok(hsv_image)
}

// ヒストグラム均等化
// e.g. let equalized_image = equalize_histogram(&gray_image)?;
pub fn equalize_histogram(image: &Mat) -> anyhow::Result<Mat> {
    let mut equalized = Mat::default();
    imgproc::equalize_hist(image, &mut equalized).context("Failed to equalize histogram")?;
    Ok(equalized)
}

// 顔認識 (Haar分類器)
// e.g. let faces = detect_faces(&image, "path/to/haarcascade_frontalface_default.xml")?;
// pub fn detect_faces(image: &Mat, cascade_path: &str) -> anyhow::Result<Vec<core::Rect>> {
//     let face_cascade =
//         objdetect::CascadeClassifier::new(cascade_path).context("Failed to load Haar cascade")?;

//     let mut gray_image = Mat::default()?;
//     imgproc::cvt_color(image, &mut gray_image, imgproc::COLOR_BGR2GRAY)
//         .context("Failed to convert image to grayscale")?;

//     let mut faces = types::VectorOfRect::new();
//     face_cascade
//         .detect_multi_scale(
//             &gray_image,
//             &mut faces,
//             1.1,
//             3,
//             0,
//             core::Size::new(30, 30),
//             core::Size::new(0, 0),
//         )
//         .context("Failed to detect faces")?;

//     Ok(faces.to_vec())
// }
