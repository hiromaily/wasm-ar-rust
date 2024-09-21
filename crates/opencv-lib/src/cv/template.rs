use anyhow::Context;
use opencv::{
    core::{self, Mat, Point},
    imgproc,
};

// execute template matching
pub fn template_matching(image: &Mat, template: &Mat) -> anyhow::Result<Mat> {
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
pub fn get_matching_result(result: &Mat) -> anyhow::Result<(Point, f64)> {
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
