// workspace
use opencv_lib::cv::{effect, grayscale, loader, saver}; // Note: change name from `opencv-lib`` to `opencv_lib``

// save grascale image
fn main() -> anyhow::Result<()> {
    // 画像を読み込む
    println!("1. load images");
    let image_path = "./images/entireimage.png";
    let image = loader::load_image(image_path)?;

    // 画像のグレースケール化を行う
    println!("2. transform images to grayscale");
    let gray_image = grayscale::convert_to_grayscale(&image)?;

    // 画像のヒストグラム均等化を行う
    println!("3. transform images to histogram");
    let gray_image = effect::equalize_histogram(&gray_image)?;

    // 保存
    let save_path = "./output/entireimage_histogram.png";
    println!("3. save image: file name {}", save_path);
    saver::save_image(save_path, &gray_image)?;

    Ok(())
}
